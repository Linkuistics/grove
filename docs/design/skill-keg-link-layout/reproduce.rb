# Design experiment: use Homebrew's actual Keg linker in temporary prefixes.
# Run with: brew ruby <this file> /absolute/path/to/ravel-2.0/bin/ravel
require "keg"
require "json"
require "tmpdir"
require "fileutils"
require "open3"
require "digest"

raise "expected one Ravel directory-delivery executable" unless ARGV.length == 1
RAVEL_SUBJECT = Pathname(ARGV.fetch(0)).realpath
raise "Ravel is not executable" unless RAVEL_SUBJECT.executable?

def check(condition, message)
  raise message unless condition
end

def fixture(root, layout, name, version = "1")
  keg = root/"Cellar"/name/version
  source = keg/(layout == :projected ? "libexec/skill-sources" : "share/ravel/skills")/name
  (source/"skills/check/resources").mkpath
  (source/"skills/check/SKILL.md").write "---\nname: check\ndescription: Fixture skill.\n---\nRead resources/note.txt.\n"
  (source/"skills/check/resources/note.txt").write "#{name} fixture resource\n"
  File.symlink("resources/note.txt", source/"skills/check/note.txt")
  if layout == :projected
    entry = keg/"share/ravel/skills"/name
    entry.dirname.mkpath
    File.symlink(source.relative_path_from(entry.dirname), entry)
  end
  [Keg.new(keg), source]
end

def resolve(front, directory, home)
  record = front.parent.parent/"share/ravel/installation.json"
  record.dirname.mkpath
  record.write JSON.generate({schemaVersion: 1, skillDirectory: directory.to_s})
  stdout, stderr, status = Open3.capture3(
    {"HOME" => home.to_s, "PATH" => "/usr/bin:/bin", "TMPDIR" => home.to_s},
    front.to_s, "skills", "resolve", "--json", unsetenv_others: true
  )
  document = JSON.parse(stdout.empty? ? stderr : stdout)
  [document, status.success?]
end

def digests(document)
  check(document["status"] == "resolved", "expected resolution: #{document}")
  document.fetch("sources").to_h { |source| [source.fetch("id"), source.fetch("resourceDigest")] }
end

subject_digest = Digest::SHA256.file(RAVEL_SUBJECT).hexdigest
experiment_digest = Digest::SHA256.file(__FILE__).hexdigest
keg_source = Pathname(Keg.instance_method(:link_dir).source_location.fetch(0))
linker_digest = Digest::SHA256.file(keg_source).hexdigest
puts "Ravel SHA-256: #{subject_digest}"
puts "Experiment SHA-256: #{experiment_digest}"
puts "Homebrew linker: #{keg_source} SHA-256: #{linker_digest}"

[:direct, :projected].product([%w[alpha beta], %w[beta alpha]]).each do |layout, order|
  Dir.mktmpdir("grove-keg-link-") do |temporary|
    root = Pathname(temporary).realpath
    # Only filesystem roots are substituted. Keg#link, #unlink, #link_dir and
    # #resolve_any_conflicts are the installed Homebrew implementations.
    {
      HOMEBREW_PREFIX: root,
      HOMEBREW_CELLAR: root/"Cellar",
      HOMEBREW_LINKED_KEGS: root/"var/homebrew/linked",
      HOMEBREW_LOCKS: root/"var/homebrew/locks",
    }.each do |name, value|
      Object.send(:remove_const, name)
      Object.const_set(name, value)
      value.mkpath
    end
    (root/"opt").mkpath
    home = root/"home"
    home.mkpath
    front = root/"resolver/bin/ravel"
    front.dirname.mkpath
    FileUtils.cp(RAVEL_SUBJECT, front)
    kegs = order.to_h { |name| [name, fixture(root, layout, name)] }
    expected = kegs.to_h do |name, (_, source)|
      document, success = resolve(front, source.parent, home)
      check(success, "reference source refused: #{document}")
      [name, digests(document).fetch(name)]
    end
    first = order.first
    kegs.fetch(first).first.link
    initial, success = resolve(front, root/"share/ravel/skills", home)
    check(success && digests(initial) == {first => expected.fetch(first)}, "first link differs")
    kegs.fetch(order.last).first.link
    document, success = resolve(front, root/"share/ravel/skills", home)
    if layout == :direct
      check(!success && document.to_s.include?("skill_resource_invalid"), "old layout did not reproduce: #{document}")
      check((root/"share/ravel/skills"/first/"skills/check/SKILL.md").symlink?, "expected expanded file link")
      puts "direct #{order.join(' -> ')}: skill_resource_invalid; first source expanded"
      kegs.fetch(first).first.unlink
      kegs.fetch(first).first.link
      repaired, success = resolve(front, root/"share/ravel/skills", home)
      check(success && digests(repaired) == expected, "manual relink failed: #{repaired}")
      puts "direct #{order.join(' -> ')}: manual relink restores both digests"
    else
      check(success && digests(document) == expected, "projection differs: #{document}")
      order.each do |name|
        entry = root/"share/ravel/skills"/name
        check(entry.symlink?, "source entry is not a link")
        check(entry.realpath == kegs.fetch(name).last, "source leaves its own payload")
        check(!(entry/"skills/check/SKILL.md").symlink?, "payload file expanded")
      end
      puts "projected #{order.join(' -> ')}: both source links and digests preserved"
      # Upgrade the first keg while the other remains linked, then remove the
      # old payload as cleanup would; the default path must reach the new one.
      kegs.fetch(first).first.unlink
      replacement, source = fixture(root, layout, first, "2")
      replacement.link
      FileUtils.rm_r(root/"Cellar"/first/"1")
      upgraded, success = resolve(front, root/"share/ravel/skills", home)
      check(success && digests(upgraded) == expected, "upgrade differs: #{upgraded}")
      check((root/"share/ravel/skills"/first).realpath == source, "upgrade retained old source")
      puts "projected #{order.join(' -> ')}: upgrade and old-keg removal preserve digests"
      kegs.fetch(order.last).first.unlink
      remaining, success = resolve(front, root/"share/ravel/skills", home)
      check(success && digests(remaining) == {first => expected.fetch(first)}, "unlink differs")
      puts "projected #{order.join(' -> ')}: uninstall leaves the other source intact"
    end
  ensure
    # Ravel makes retained snapshots read-only. Restore permissions only on
    # this experiment's real directories; do not follow any fixture symlink.
    root.find { |path| path.chmod(path.stat.mode | 0700) if !path.symlink? && path.directory? }
  end
end
check(Digest::SHA256.file(RAVEL_SUBJECT).hexdigest == subject_digest, "Ravel subject changed during measurement")
check(Digest::SHA256.file(__FILE__).hexdigest == experiment_digest, "Experiment changed during measurement")
check(Digest::SHA256.file(keg_source).hexdigest == linker_digest, "Homebrew linker changed during measurement")
puts "keg-link-layout: all assertions passed; measured subjects unchanged"
