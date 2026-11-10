# The shared default directory seam: actual formula output, linked by
# Homebrew's own linker into private prefixes, then resolved by Ravel.
#
#   brew ruby scripts/shared-directory-seam.rb PLAN
#
# scripts/shared-directory-seam.sh writes PLAN, a JSON document that names the
# Ravel formula and archive, each formula under test and each published
# formula, with their archives, declared layouts and accepted values.
# "Shared default directory acceptance" of docs/specs/codex-skill-provisioning.md
# owns the cases and the comparisons.
#
# Each case owns a new private prefix and HOME. Only Homebrew's path constants
# change. The install methods, Cleaner#clean, Keg#link, Keg#unlink, Keg#optlink
# and Keg#uninstall are the installed Homebrew implementations. The seam calls
# them in the order of Homebrew's own callers:
#
# - FormulaInstaller#install runs the install method, then #clean runs
#   Cleaner#clean, then #finish calls #link, which calls Keg#link. A keg-only
#   formula gets Keg#optlink only. Keg#link rolls back a failed link with
#   Keg#unlink of the whole keg, and FormulaInstaller#link reports
#   Keg::ConflictError without a retry.
# - Install.install_formula unlinks each outdated keg before the upgrade. Its
#   ensure block relinks them only when the new version is not installed.
# - Cleanup#cleanup_keg removes an old keg with Keg#uninstall.
# - Uninstall.uninstall_kegs calls Keg#unlink and then Keg#uninstall.
#
# https://github.com/Homebrew/brew/blob/bc63375ac3a4d6debb1ceb441ae6e22854f0909a/Library/Homebrew/formula_installer.rb
# https://github.com/Homebrew/brew/blob/bc63375ac3a4d6debb1ceb441ae6e22854f0909a/Library/Homebrew/install.rb
# https://github.com/Homebrew/brew/blob/bc63375ac3a4d6debb1ceb441ae6e22854f0909a/Library/Homebrew/keg.rb
require "cleaner"
require "digest"
require "find"
require "fileutils"
require "formula"
require "formulary"
require "json"
require "keg"
require "open3"
require "pkg_version"

raise "expected one PLAN file" unless ARGV.length == 1

PLAN = JSON.parse(File.read(ARGV.fetch(0)))
SCRATCH = Pathname(PLAN.fetch("scratch")).realpath
RECORD = Pathname(PLAN.fetch("record"))
EMPTY_SHA256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".freeze

class SeamFailure < StandardError; end

def check(condition, message)
  raise SeamFailure, message unless condition
end

def say(text)
  puts "shared-directory-seam: #{text}"
end

def sha256(path)
  Digest::SHA256.file(path).hexdigest
end

# A keg's directory. Keg#path is protected; Keg#to_s is public.
def root_of(keg)
  Pathname(keg.to_s)
end

# One formula file at one package version: a formula under test, a published
# formula, Ravel's formula or a fixture.
class Variant
  attr_reader :role, :name, :path, :archive, :layout, :sources, :klass, :accepted

  def initialize(role:, name:, path:, archive:, layout:, sources:, accepted: nil, bin: [])
    @role = role
    @name = name
    @path = Pathname(path)
    @archive = archive && Pathname(archive)
    @layout = layout
    @sources = sources
    @accepted = accepted
    @bin = bin
    # Each file loads into its own namespace, so a published formula and its
    # revision load side by side.
    @klass = Formulary.load_formula_from_path(name, @path, flags: [], ignore_errors: false)
  end

  attr_reader :bin
  attr_writer :accepted

  def formula
    @klass.new(@name, @path, :stable)
  end

  # The full package version, with the revision, as Homebrew labels the keg.
  def label
    @label ||= formula.pkg_version.to_s
  end

  # The label of the next revision of the same version.
  def revision_label
    f = formula
    PkgVersion.new(f.version, f.revision + 1).to_s
  end

  def describe
    "#{@name} #{label} (#{@role}, #{@layout || 'ravel'})"
  end
end

# The fixture formulae. Their content is generated here and digested with the
# other formula files, so the record identifies each one.
FIXTURE_TEMPLATE = <<~'RUBY'.freeze
  class @CLASS@ < Formula
    desc "Fixture skill formula of Grove's shared-directory seam"
    homepage "https://github.com/Linkuistics/grove"
    url "file:///dev/null"
    version "@VERSION@"
    sha256 "@EMPTY@"

    SOURCES = %w[@SOURCES@].freeze
    LAYOUT = "@LAYOUT@".freeze
    BIN = %w[@BIN@].freeze

    def install
      SOURCES.each do |id|
        source = (LAYOUT == "projected") ? libexec/"skill-sources"/id : share/"ravel/skills"/id
        skill = source/"skills/check"
        (skill/"resources").mkpath
        (skill/"scripts").mkpath
        (skill/"SKILL.md").write "---\nname: check\ndescription: Fixture skill of #{id}.\n---\nRead resources/note.txt.\n"
        (skill/"resources/note.txt").write "#{id} fixture resource of #{name} #{version}\n"
        (skill/"scripts/run.sh").write "#!/bin/sh\necho #{id}\n"
        chmod 0755, skill/"scripts/run.sh"
        ln_s "resources/note.txt", skill/"note.txt"
        (share/"ravel/skills").install_symlink source if LAYOUT == "projected"
      end
      bin.mkpath unless BIN.empty?
      BIN.each do |command|
        (bin/command).write "#!/bin/sh\nexit 0\n"
        chmod 0755, bin/command
      end
    end
  end
RUBY

def fixture(name:, version:, layout:, sources:, bin: [])
  path = SCRATCH/"fixtures/#{name}-#{version}/#{name}.rb"
  path.dirname.mkpath
  path.write FIXTURE_TEMPLATE.gsub("@CLASS@", Formulary.class_s(name))
                             .gsub("@VERSION@", version)
                             .gsub("@EMPTY@", EMPTY_SHA256)
                             .gsub("@SOURCES@", sources.join(" "))
                             .gsub("@LAYOUT@", layout)
                             .gsub("@BIN@", bin.join(" "))
  Variant.new(role: "fixture", name:, path:, archive: nil, layout:, sources:, bin:)
end

def read_accepted(file)
  File.readlines(file, chomp: true).reject(&:empty?).to_h do |line|
    id, digest, roots = line.split("\t")
    check(id && digest && roots, "#{file}: expected ID, resourceDigest and roots, tab-separated: #{line}")
    [id, { "digest" => digest, "roots" => roots.split(",").sort }]
  end
end

def variant_from(entry)
  accepted = entry["accepted"] && read_accepted(entry["accepted"])
  Variant.new(role: entry.fetch("role"), name: entry.fetch("name"), path: entry.fetch("formula"),
              archive: entry["archive"], layout: entry.fetch("layout"), sources: entry.fetch("sources"),
              accepted:)
end

RAVEL_PLAN = PLAN.fetch("ravel")
RAVEL = Variant.new(role: "ravel", name: "ravel", path: RAVEL_PLAN.fetch("formula"),
                    archive: RAVEL_PLAN.fetch("archive"), layout: nil, sources: [])
FORMULAE = PLAN.fetch("formulae").map { |entry| variant_from(entry) }

SUBJECTS = FORMULAE.select { |v| v.role == "subject" }
PUBLISHED = FORMULAE.select { |v| v.role == "published" }
PROJECTED_REAL = SUBJECTS.select { |v| v.layout == "projected" }
# A direct formula under test joins the failure control unless its published
# formula is the same file.
DIRECT_REAL = PUBLISHED.select { |v| v.layout == "direct" } +
              SUBJECTS.select do |v|
                v.layout == "direct" &&
                  PUBLISHED.none? { |p| p.name == v.name && sha256(p.path) == sha256(v.path) }
              end
DUPLICATE_ID = PROJECTED_REAL.first&.sources&.first || "seam-fixture"

FIXTURE_OLD = fixture(name: "seam-fixture", version: "1", layout: "direct", sources: ["seam-fixture"])
FIXTURE_NEW = fixture(name: "seam-fixture", version: "2", layout: "projected", sources: ["seam-fixture"])
CONTROL = fixture(name: "seam-control", version: "1", layout: "direct", sources: ["seam-control"])
DUPLICATE_OLD = fixture(name: "seam-duplicate", version: "1", layout: "projected",
                        sources: ["seam-duplicate"], bin: ["seam-duplicate"])
DUPLICATE_NEW = fixture(name: "seam-duplicate", version: "2", layout: "projected",
                        sources: ["seam-duplicate", DUPLICATE_ID], bin: ["seam-duplicate"])

# The projected kegs of the ordinary cases. The duplicate fixture's first
# version gives two projected fixture kegs, and it has an executable.
PROJECTED = PROJECTED_REAL + [FIXTURE_NEW, DUPLICATE_OLD]
# Each migration pair: a published direct formula and its projected revision.
MIGRATIONS = PROJECTED_REAL.filter_map do |subject|
  previous = PUBLISHED.find { |p| p.name == subject.name && p.layout == "direct" }
  [previous, subject] if previous
end + [[FIXTURE_OLD, FIXTURE_NEW]]
MIGRATIONS.each do |previous, subject|
  check(previous.label != subject.label,
        "#{subject.name}: the revision has the published package version #{subject.label}; " \
        "a layout change needs a new revision or release")
end

HOMEBREW_FILES = %w[keg.rb formula_installer.rb install.rb cleaner.rb cleanup.rb uninstall.rb extend/pathname.rb]
                 .map { |file| HOMEBREW_LIBRARY_PATH/file }

# Every file that this run reads: the instrument, Homebrew's linker, installer
# and cleaner, each formula file, archive and file of accepted values.
def subjects
  files = PLAN.fetch("instrument") + HOMEBREW_FILES.map(&:to_s) +
          ([RAVEL] + FORMULAE + [FIXTURE_OLD, FIXTURE_NEW, CONTROL, DUPLICATE_OLD, DUPLICATE_NEW])
          .flat_map { |v| [v.path, v.archive].compact.map(&:to_s) } +
          PLAN.fetch("formulae").filter_map { |entry| entry["accepted"] }
  files.uniq.to_h { |file| [file, sha256(file)] }
end

# Homebrew's path constants for one private prefix.
PATH_CONSTANTS = {
  HOMEBREW_PREFIX:      "prefix",
  HOMEBREW_CELLAR:      "prefix/Cellar",
  HOMEBREW_LINKED_KEGS: "prefix/var/homebrew/linked",
  HOMEBREW_PINNED_KEGS: "prefix/var/homebrew/pinned",
  HOMEBREW_LOCKS:       "prefix/var/homebrew/locks",
  HOMEBREW_TEMP_CELLAR: "prefix/var/homebrew/tmp/.cellar",
  HOMEBREW_CASKROOM:    "prefix/Caskroom",
  HOMEBREW_CACHE:       "cache",
  HOMEBREW_LOGS:        "logs",
}.freeze

# One case: a new private prefix and HOME.
class Case
  attr_reader :name, :steps

  def initialize(name)
    @name = name
    @root = SCRATCH/"cases/#{name}"
    check(!@root.exist?, "case #{name} already ran")
    @root.mkpath
    @root = @root.realpath
    PATH_CONSTANTS.each do |constant, relative|
      Object.send(:remove_const, constant)
      Object.const_set(constant, (@root/relative).freeze)
    end
    Object.send(:remove_const, :HOMEBREW_CACHE_FORMULA)
    Object.const_set(:HOMEBREW_CACHE_FORMULA, (HOMEBREW_CACHE/"Formula").freeze)
    [HOMEBREW_CELLAR, HOMEBREW_PREFIX/"opt", HOMEBREW_LINKED_KEGS, HOMEBREW_LOCKS, HOMEBREW_CACHE].each(&:mkpath)
    @home = @root/"home"
    @tmp = @root/"tmp"
    @home.mkpath
    @tmp.mkpath
    @builds = 0
    @steps = []
    @kegs = {}
    @pristine = {}
    @ravel = nil
    @ravel_linked = false
  end

  def step(action, result)
    @steps << { "action" => action, "result" => result }
    say "#{@name}: #{action}: #{result}"
  end

  # FormulaInstaller#install and #clean: the install method in an extracted
  # archive, its buildpath, then Homebrew's cleaner.
  def install(variant, label = variant.label)
    keg_path = HOMEBREW_CELLAR/variant.name/label
    check(!keg_path.exist?, "#{keg_path} already exists")
    @builds += 1
    build = @root/"build/#{@builds}"
    build.mkpath
    buildpath = build
    if variant.archive
      check(system("tar", "-xf", variant.archive.to_s, "-C", build.to_s), "cannot extract #{variant.archive}")
      # Homebrew stages a single top directory as the buildpath.
      buildpath = build.children.first if build.children.length == 1 && build.children.first.directory?
    end
    formula = variant.formula
    formula.define_singleton_method(:prefix) { |*_args| keg_path }
    # Homebrew's stage creates the build HOME, .brew_home, in the buildpath.
    # https://github.com/Homebrew/brew/blob/bc63375ac3a4d6debb1ceb441ae6e22854f0909a/Library/Homebrew/formula.rb
    formula.instance_variable_set(:@buildpath, buildpath)
    (buildpath/".brew_home").mkpath
    (buildpath/".brew_home/.gitignore").write "*"
    keg_path.mkpath
    Dir.chdir(buildpath) { formula.install }
    Cleaner.new(formula).clean
    keg = Keg.new(keg_path)
    check_layout(variant, keg)
    @pristine[keg_path.to_s] = variant.sources.to_h { |id| [id, inventory(keg_path/"share/ravel/skills"/id)] }
    accept(variant, keg) if variant.accepted.nil? && !variant.layout.nil?
    keg
  end

  # The installed keg matches its formula's declared layout.
  def check_layout(variant, keg)
    if variant.layout.nil?
      record = root_of(keg)/"share/ravel/installation.json"
      %w[examples notices].each do |entry|
        check((root_of(keg)/"share/ravel"/entry).directory?, "Ravel's keg holds no share/ravel/#{entry}")
      end
      check(record.file?, "Ravel's keg holds no share/ravel/installation.json")
      document = JSON.parse(record.read)
      check(document == { "schemaVersion" => 1, "skillDirectory" => (HOMEBREW_PREFIX/"share/ravel/skills").to_s },
            "Ravel's installation record names #{document}, not this prefix's share/ravel/skills")
      check(!(root_of(keg)/"share/ravel/skills").exist?, "Ravel's keg holds share/ravel/skills")
      return
    end
    skills = root_of(keg)/"share/ravel/skills"
    check(skills.directory?, "#{variant.describe}: the keg holds no share/ravel/skills")
    actual = skills.children.map { |entry| entry.basename.to_s }.sort
    check(actual == variant.sources.sort, "#{variant.describe}: the keg holds the sources #{actual}, not #{variant.sources}")
    variant.sources.each do |id|
      entry = skills/id
      payload = root_of(keg)/"libexec/skill-sources"/id
      if variant.layout == "projected"
        check(entry.symlink? && !entry.readlink.absolute?,
              "#{variant.describe}: share/ravel/skills/#{id} is not a relative symlink, so the keg is not projected")
        check(payload.directory? && !payload.symlink?, "#{variant.describe}: libexec/skill-sources/#{id} is not a directory")
        check(entry.realpath == payload.realpath, "#{variant.describe}: share/ravel/skills/#{id} leaves its private source")
      else
        check(entry.directory? && !entry.symlink?,
              "#{variant.describe}: share/ravel/skills/#{id} is not a real directory, so the keg is not direct")
        check(!payload.exist?, "#{variant.describe}: a direct keg holds libexec/skill-sources/#{id}")
      end
    end
  end

  # Each entry under the source root, with its type and executable bit. The
  # root is followed once; nothing below it is.
  def inventory(entry)
    root = entry.realpath
    result = {}
    Find.find(root.to_s) do |name|
      next if name == root.to_s

      path = Pathname(name)
      stat = path.lstat
      result[path.relative_path_from(root).to_s] =
        if stat.symlink? then "link #{path.readlink}"
        elsif stat.directory? then "directory"
        elsif stat.file? then stat.mode.anybits?(0o111) ? "executable" : "file"
        else "other"
        end
    end
    result
  end

  # Accepted values from the pristine private payloads, through settings that
  # name each one, for a formula whose caller has no plain checkout.
  def accept(variant, keg)
    home = @root/"accept-home/#{variant.name}-#{variant.label}"
    (home/".config/ravel").mkpath
    sources = variant.sources.map do |id|
      path = (root_of(keg)/"share/ravel/skills"/id).realpath
      roots = %w[skills craft].select { |root| (path/root).directory? }
      { "id" => id, "type" => "local", "path" => path.to_s, "roots" => roots }
    end
    (home/".config/ravel/settings.json").write JSON.generate({ "skills" => { "schemaVersion" => 1, "sources" => sources } })
    document, success = run_resolve(ravel_front, home)
    check(success && document["status"] == "resolved", "#{variant.describe}: its pristine sources do not resolve: #{document}")
    variant.accepted = facts(document).transform_values { |fact| fact.slice("digest", "roots") }
    step("accept #{variant.describe}", "accepted values from its pristine private payloads")
  end

  def ravel_front
    @ravel_linked ? HOMEBREW_PREFIX/"bin/ravel" : HOMEBREW_PREFIX/"opt/ravel/bin/ravel"
  end

  def run_resolve(front, home)
    stdout, stderr, status = Open3.capture3(
      { "HOME" => home.to_s, "TMPDIR" => @tmp.to_s, "PATH" => "/usr/bin:/bin" },
      front.to_s, "skills", "resolve", "--json", unsetenv_others: true
    )
    text = stdout.strip.empty? ? stderr : stdout
    line = text.lines.reverse.find { |candidate| candidate.start_with?("{") }
    check(line, "Ravel printed no JSON document: #{stdout}#{stderr}")
    [JSON.parse(line), status.success?]
  end

  # The id, digest, roots and originPath of each resolved source.
  def facts(document)
    document.fetch("sources").to_h do |source|
      roots = document.fetch("directories").select { |d| d["source"] == source["id"] }.map do |directory|
        Pathname(directory["path"]).relative_path_from(Pathname(source["snapshotRoot"])).to_s
      end
      [source["id"], { "digest" => source["resourceDigest"], "roots" => roots.sort, "origin" => source["originPath"] }]
    end
  end

  # Ravel's keg, keg-only (Keg#optlink) unless the case links it.
  def install_ravel(link:)
    @ravel = install(RAVEL)
    if link
      link(@ravel, RAVEL)
    else
      @ravel.optlink
      step("install #{RAVEL.describe}", "keg-only: opt link only")
    end
  end

  # FormulaInstaller#link. Keg#link unlinks the whole keg when any entry
  # conflicts.
  def link(keg, variant)
    keg.link
    @ravel_linked = true if variant.equal?(RAVEL)
    @kegs[variant.name] = [keg, variant] unless variant.equal?(RAVEL)
    step("link #{keg_name(keg, variant)}", "linked")
    :linked
  rescue Keg::ConflictError => e
    step("link #{keg_name(keg, variant)}", "Keg::ConflictError at #{e.dst.relative_path_from(HOMEBREW_PREFIX)}; " \
                                     "Keg#link unlinked the whole keg")
    :conflict
  end

  # The keg's Cellar label, which differs from the formula's for a revision.
  def keg_name(keg, variant)
    "#{variant.name} #{root_of(keg).basename} (#{variant.role}, #{variant.layout || 'ravel'})"
  end

  def install_and_link(variant)
    link(install(variant), variant)
  end

  # Link each variant in ORDER. Each must link.
  def link_all(order)
    order.each do |variant|
      check(install_and_link(variant) == :linked, "#{@name}: #{variant.describe} did not link")
      yield variant if block_given?
    end
  end

  # Promotion: link the keg-only Ravel keg.
  def promote_ravel
    check(link(@ravel, RAVEL) == :linked, "#{@name}: Ravel did not link")
  end

  attr_reader :ravel

  def keg(variant)
    @kegs.fetch(variant.name).first
  end

  # Install.install_formula for an upgrade, then Cleanup#cleanup_keg.
  def upgrade(variant, label = variant.label, cleanup: true)
    old_keg, = @kegs.fetch(variant.name)
    old_keg.unlink
    @kegs.delete(variant.name)
    new_keg = install(variant, label)
    result = link(new_keg, variant)
    # Install.install_formula relinks the outdated kegs only when the new
    # version is not installed. It is installed here, linked or not.
    check(root_of(new_keg).directory?, "the new keg of #{variant.name} is not installed")
    if cleanup
      old_keg.uninstall(raise_failures: true)
      step("clean up #{root_of(old_keg).relative_path_from(HOMEBREW_CELLAR)}", "Keg#uninstall removed the old keg")
    end
    [new_keg, result]
  end

  # Uninstall.uninstall_kegs.
  def uninstall(variant)
    keg, = @kegs.delete(variant.name)
    keg.unlink
    keg.uninstall
    step("uninstall #{keg_name(keg, variant)}", "Keg#unlink and Keg#uninstall")
  end

  # Resolve the shared default directory with no settings, and compare each
  # source with its accepted values and its owning private payload.
  def verify(context)
    check_no_dangling_link
    document, success = run_resolve(ravel_front, @home)
    expected = @kegs.flat_map { |_, (keg, variant)| variant.sources.map { |id| [id, [keg, variant]] } }.to_h
    if expected.empty?
      check(success && (document["sources"] || []).empty?, "#{context}: expected no source, Ravel reported #{document}")
      step("resolve after #{context}", "no source (#{document['status']})")
      return
    end
    check(success && document["status"] == "resolved", "#{context}: the shared directory does not resolve: #{document}")
    actual = facts(document)
    check(actual.keys.sort == expected.keys.sort, "#{context}: Ravel resolved #{actual.keys.sort}, not #{expected.keys.sort}")
    actual.each do |id, fact|
      keg, variant = expected.fetch(id)
      accepted = variant.accepted.fetch(id)
      check(fact["digest"] == accepted["digest"],
            "#{context}: #{id} has resourceDigest #{fact['digest']}, not the accepted #{accepted['digest']}")
      check(fact["roots"] == accepted["roots"], "#{context}: #{id} has roots #{fact['roots']}, not #{accepted['roots']}")
      payload = root_of(keg)/"libexec/skill-sources"/id
      payload = payload.realpath if payload.exist?
      check(fact["origin"] == payload.to_s, "#{context}: #{id} has originPath #{fact['origin']}, not #{payload}")
      entry = HOMEBREW_PREFIX/"share/ravel/skills"/id
      check(entry.symlink?, "#{context}: the prefix's share/ravel/skills/#{id} is not a link")
      check(inventory(entry) == @pristine.fetch(root_of(keg).to_s).fetch(id),
            "#{context}: the files of #{id} through the prefix differ from its pristine payload")
    end
    step("resolve after #{context}", "#{actual.length} sources: IDs, roots, digests, origins and payload files as accepted")
  end

  # The failure that the projection removes.
  def expect_expansion(context, keg, variant)
    document, success = run_resolve(ravel_front, @home)
    check(!success && document.to_s.include?("skill_resource_invalid"),
          "#{context}: expected skill_resource_invalid, Ravel reported #{document}")
    id = variant.sources.first
    entry = HOMEBREW_PREFIX/"share/ravel/skills"/id
    check(entry.directory? && !entry.symlink?, "#{context}: #{id} is not an expanded directory in the prefix")
    links = inventory(entry).values.count { |type| type.start_with?("link ") }
    pristine_links = @pristine.fetch(root_of(keg).to_s).fetch(id).values.count { |type| type.start_with?("link ") }
    check(links > pristine_links, "#{context}: #{id} holds no file link that Homebrew made")
    step("resolve after #{context}", "skill_resource_invalid; #{variant.describe} expanded into file links")
  end

  # No link in the prefix outside the Cellar fails to resolve.
  def check_no_dangling_link
    each_prefix_link do |path|
      check(path.exist?, "the prefix holds the dangling link #{path.relative_path_from(HOMEBREW_PREFIX)}")
    end
  end

  def each_prefix_link
    skipped = [HOMEBREW_CELLAR, HOMEBREW_PREFIX/"var", HOMEBREW_CASKROOM].map(&:to_s)
    Find.find(HOMEBREW_PREFIX.to_s) do |name|
      Find.prune if skipped.include?(name)
      path = Pathname(name)
      yield path if path.symlink?
    end
  end

  # The link directories hold no entry of KEG. Its opt link may remain.
  def expect_no_entry_of(context, keg, variant)
    entries = []
    each_prefix_link do |path|
      next if path.dirname == HOMEBREW_PREFIX/"opt"

      target = (path.dirname.realpath/path.readlink).cleanpath
      entries << path.relative_path_from(HOMEBREW_PREFIX).to_s if target.to_s.start_with?("#{root_of(keg)}/")
    end
    check(entries.empty?, "#{context}: the prefix still links #{entries} into the refused keg")
    (variant.sources.map { |id| "share/ravel/skills/#{id}" } + variant.bin.map { |c| "bin/#{c}" }).each do |entry|
      path = HOMEBREW_PREFIX/entry
      next unless path.symlink? || path.exist?

      check(path.realpath.to_s.start_with?("#{HOMEBREW_CELLAR}/") && !path.realpath.to_s.start_with?("#{root_of(keg)}/"),
            "#{context}: the prefix's #{entry} belongs to the refused keg")
    end
    step("inspect after #{context}", "no source or executable entry of #{variant.describe} in the prefix")
  end

  def finish
    # Ravel makes retained snapshots read-only. Restore permissions only on
    # this case's real directories, without following a link.
    @root.find { |path| path.chmod(path.stat.mode | 0o700) if !path.symlink? && path.directory? }
    { "name" => @name, "result" => "passed", "steps" => @steps }
  end
end

def run_case(name)
  kase = Case.new(name)
  yield kase
  kase.finish
end

# Link each projected keg in ORDER, from an empty prefix.
def order_case(name, order)
  run_case(name) do |c|
    check(!(HOMEBREW_PREFIX/"share").exist?, "#{name}: the new prefix already holds share")
    c.step("prefix", "no share, share/ravel or share/ravel/skills")
    c.install_ravel(link: false)
    c.link_all(order) { |variant| c.verify("linking #{variant.describe}") }
  end
end

def failure_case(name, order)
  run_case(name) do |c|
    c.install_ravel(link: false)
    c.link_all(order)
    c.expect_expansion("linking #{order.map(&:describe).join(', ')}", c.keg(order.first), order.first)
  end
end

# Each file of Ravel's share/ravel reaches Ravel's keg through the prefix. A
# displaced directory link becomes a real directory of file links, so the
# comparison is file by file.
def ravel_entries_linked(kase, context, keg)
  shared = root_of(keg)/"share/ravel"
  files = Dir.glob("**/*", base: shared).reject { |entry| (shared/entry).directory? }.sort
  check(files.include?("installation.json") && files.any? { |f| f.start_with?("examples/") } &&
        files.any? { |f| f.start_with?("notices/") }, "Ravel's keg lacks a shared entry: #{files}")
  files.each do |entry|
    path = HOMEBREW_PREFIX/"share/ravel"/entry
    check(path.exist? && path.realpath == (shared/entry).realpath,
          "#{context}: the prefix's share/ravel/#{entry} does not reach Ravel's keg")
  end
  kase.step("inspect after #{context}", "each of #{files.length} files of Ravel's share/ravel reaches Ravel's keg")
end

say "Homebrew #{HOMEBREW_VERSION} at #{PLAN.fetch('homebrewRevision')}"
say "Ravel: #{RAVEL.describe}, archive #{RAVEL_PLAN.fetch('archiveSha256')}, lane #{RAVEL_PLAN.fetch('lane')}, " \
    "build identity #{RAVEL_PLAN.fetch('buildIdentity')}"
(FORMULAE + [FIXTURE_OLD, FIXTURE_NEW, CONTROL, DUPLICATE_OLD, DUPLICATE_NEW]).each do |variant|
  say "formula: #{variant.describe}, sources #{variant.sources.join(' ')}"
end
DIRECT = DIRECT_REAL + [FIXTURE_OLD, CONTROL]
say "failure control: #{DIRECT.map(&:describe).join(', ')}"
say "projected: #{PROJECTED.map(&:describe).join(', ')}"
say "migrations: #{MIGRATIONS.map { |a, b| "#{a.describe} -> #{b.label}" }.join(', ')}"
say "duplicate ID: #{DUPLICATE_ID}"

before = subjects
record = {
  "schemaVersion" => 1,
  "homebrew" => { "version" => HOMEBREW_VERSION, "revision" => PLAN.fetch("homebrewRevision") },
  "tap" => PLAN.fetch("tap"),
  "ravel" => RAVEL_PLAN.merge("packageVersion" => RAVEL.label),
  "formulae" => ([RAVEL] + FORMULAE + [FIXTURE_OLD, FIXTURE_NEW, CONTROL, DUPLICATE_OLD, DUPLICATE_NEW]).map do |v|
    origin = v.equal?(RAVEL) ? RAVEL_PLAN["from"] : PLAN.fetch("formulae").find { |e| e["formula"] == v.path.to_s }&.fetch("from")
    { "name" => v.name, "role" => v.role, "packageVersion" => v.label, "layout" => v.layout, "from" => origin,
      "sources" => v.sources, "formula" => v.path.to_s, "formulaSha256" => sha256(v.path),
      "archiveSha256" => v.archive && sha256(v.archive) }
  end,
  "subjects" => before,
  "cases" => [],
}
begin
  cases = record["cases"]
  # The failure control: direct kegs, a published formula first where one
  # exists.
  cases << failure_case("failure-control-forward", DIRECT_REAL + [FIXTURE_OLD, CONTROL])
  cases << failure_case("failure-control-reverse", DIRECT_REAL.reverse + [CONTROL, FIXTURE_OLD])
  # Both installation orders, each from a prefix without shared ancestors.
  cases << order_case("order-forward", PROJECTED)
  cases << order_case("order-reverse", PROJECTED.reverse)
  # A fresh Grove installation links its Ravel dependency first.
  cases << run_case("ravel-first") do |c|
    c.install_ravel(link: true)
    ravel_entries_linked(c, "linking Ravel", c.ravel)
    c.link_all(PROJECTED) { |variant| c.verify("linking #{variant.describe}") }
    ravel_entries_linked(c, "linking every source", c.ravel)
  end
  # Promotion links Ravel beside the installed sources.
  cases << run_case("ravel-promotion") do |c|
    c.install_ravel(link: false)
    c.link_all(PROJECTED)
    c.verify("linking every source with Ravel keg-only")
    c.promote_ravel
    c.verify("promoting Ravel")
    ravel_entries_linked(c, "promoting Ravel", c.ravel)
  end
  # A revision upgrade of each keg, the old keg's cleanup, then uninstall.
  cases << run_case("upgrade-cleanup-uninstall") do |c|
    c.install_ravel(link: true)
    c.link_all(PROJECTED)
    c.verify("linking every source")
    PROJECTED.each do |variant|
      _, result = c.upgrade(variant, variant.revision_label)
      check(result == :linked, "#{variant.name} #{variant.revision_label} did not link")
      c.verify("upgrading #{variant.name} to #{variant.revision_label} and removing the old keg")
    end
    PROJECTED.each do |variant|
      c.uninstall(variant)
      c.verify("uninstalling #{variant.name}")
    end
  end
  # Direct kegs beside projected ones, then each direct keg upgraded to its
  # projected revision.
  cases << run_case("mixed-layout-migration") do |c|
    c.install_ravel(link: true)
    previous = MIGRATIONS.map(&:first)
    rest = PROJECTED - MIGRATIONS.map(&:last)
    c.link_all(previous + rest)
    if rest.empty?
      c.step("mixed set", "no projected keg links after the direct kegs")
    else
      c.expect_expansion("linking projected kegs beside direct kegs", c.keg(previous.first), previous.first)
    end
    MIGRATIONS.each_with_index do |(_, subject), index|
      _, result = c.upgrade(subject)
      check(result == :linked, "#{subject.describe} did not link")
      if index == MIGRATIONS.length - 1
        c.verify("upgrading the last direct keg, #{subject.name}, to #{subject.label}")
      else
        c.step("upgrade #{subject.name}", "resolution waits until every direct keg is upgraded")
      end
    end
  end
  # A later keg that ships an installed source ID, fresh and as an upgrade.
  owners = PROJECTED - [DUPLICATE_OLD]
  cases << run_case("duplicate-fresh") do |c|
    c.install_ravel(link: true)
    c.link_all(owners)
    c.verify("linking every source")
    keg = c.install(DUPLICATE_NEW)
    check(c.link(keg, DUPLICATE_NEW) == :conflict, "#{DUPLICATE_NEW.describe} linked beside #{DUPLICATE_ID}")
    c.verify("refusing #{DUPLICATE_NEW.describe}")
    c.expect_no_entry_of("refusing #{DUPLICATE_NEW.describe}", keg, DUPLICATE_NEW)
  end
  cases << run_case("duplicate-upgrade") do |c|
    c.install_ravel(link: true)
    c.link_all(owners + [DUPLICATE_OLD])
    c.verify("linking every source")
    keg, result = c.upgrade(DUPLICATE_NEW, cleanup: false)
    check(result == :conflict, "#{DUPLICATE_NEW.describe} linked beside #{DUPLICATE_ID}")
    c.verify("refusing the upgrade to #{DUPLICATE_NEW.describe}")
    c.expect_no_entry_of("refusing the upgrade", keg, DUPLICATE_NEW)
    old = Keg.new(HOMEBREW_CELLAR/DUPLICATE_OLD.name/DUPLICATE_OLD.label)
    check(!old.linked?, "the earlier keg of #{DUPLICATE_OLD.name} is linked again")
    c.expect_no_entry_of("refusing the upgrade", old, DUPLICATE_OLD)
  end
  after = subjects
  changed = before.keys.reject { |file| after[file] == before[file] }
  check(changed.empty?, "measured subjects changed during the run: #{changed.join(', ')}")
  record["result"] = "passed"
  say "all #{cases.length} cases passed; measured subjects unchanged"
rescue SeamFailure => e
  record["result"] = "failed"
  record["failure"] = e.message
  warn "shared-directory-seam: FAILED: #{e.message}"
ensure
  RECORD.dirname.mkpath
  RECORD.write "#{JSON.pretty_generate(record)}\n"
  say "record: #{RECORD}"
end
exit(record["result"] == "passed" ? 0 : 1)
