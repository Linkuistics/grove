# Exercise the production formula with Homebrew itself in an owned prefix.
# Adapted from Ravel's installed-formula seam; no mocked Formula DSL.
# https://docs.brew.sh/rubydoc/Formula.html#test-instance_method
require "formula"
require "formula_assertions"

formula_path, source, prefix_path, test_path, ravel_1_1, ravel_2_0 = ARGV
raise "expected FORMULA SOURCE PREFIX TESTPATH RAVEL_1_1 RAVEL_2_0" unless ARGV.length == 6
prefix_path = Pathname(prefix_path)
load formula_path
# https://docs.brew.sh/rubydoc/Formula.html#initialize-instance_method
formula = Grove.new("grove", Pathname(formula_path), :stable)
formula.extend(Homebrew::Assertions)
formula.define_singleton_method(:prefix) { |*_args| prefix_path }
# Every entry below BASE that PATTERN matches, dot entries such as a source's
# .claude-plugin included, without the `.` that FNM_DOTMATCH adds.
def entries(pattern, base)
  Dir.glob(pattern, File::FNM_DOTMATCH, base: base).reject { |entry| File.basename(entry) == "." }.sort
end
# The keg root also holds the marketplace files of the hook adapters.
expected = ([".agents", ".claude-plugin", "bin", "libexec", "share"] +
            entries("{.agents,.claude-plugin,bin,libexec,share}/**/*", source)).sort
raise "the archive holds no skill sources" unless expected.include?("share/ravel/skills/grove-core")
%w[.claude-plugin/marketplace.json .agents/plugins/marketplace.json].each do |file|
  raise "the archive holds no #{file}" unless expected.include?(file)
end
Dir.chdir(source) { formula.install }
actual = entries("**/*", prefix_path)
raise "formula installation differs from archive: #{(actual - expected) + (expected - actual)}" unless actual == expected
# The caveats show both configuration formats, the default skill directory and
# the explicit ravel@2 prefix, and each sample they copy is installed.
# https://docs.brew.sh/rubydoc/Formula.html#caveats-instance_method
caveats = formula.caveats.to_s
{
  "the policy.ts sample" => "ravel-policy/sample/policy.ts ~/.config/ravel/policy.ts",
  "the config.rhai sample" => "ravel-policy/sample/config.rhai ~/.config/ravel/config.rhai",
  "the default skill directory" => "skill directory, $(brew --prefix)/share/ravel/skills.",
  "the settings without sources" => "names no\n\"sources\", also when that file is absent",
  "the ravel@2 prefix" => 'PATH="$(brew --prefix ravel@2)/bin:$PATH" grove',
  # The four commands of "Hook adapters" in docs/specs/codex-skill-provisioning.md.
  "the Claude Code hooks marketplace" => 'claude plugin marketplace add "$(brew --prefix)/opt/grove"',
  "the Claude Code hooks plugin" => "claude plugin install technical-communication-hooks@writing-hooks",
  "the guardrail marketplace" => 'claude plugin marketplace add "$(brew --prefix)/opt/personal"',
  "the guardrail plugin" => "claude plugin install guardrail@personal",
  "the Codex hooks marketplace" => 'codex plugin marketplace add "$(brew --prefix)/opt/grove"',
  "the Codex hooks plugin" => "codex plugin add technical-communication@writing-hooks",
  "the Codex hook review" => 'choose "Trust all',
  "the Pi extension" => 'ln -s "$(brew --prefix)/share/ravel/skills/writing/craft/technical-communication/' \
                        'harnesses/pi/technical-communication.ts" ~/.pi/agent/extensions/',
}.each do |subject, text|
  raise "the caveats do not show #{subject}: #{text}" unless caveats.include?(text)
end
# The retired routes: settings that name checkouts, and the manual installer.
["`skills` object", "checkout", "install.sh", "@linkuistics"].each do |text|
  raise "the caveats still show a retired route: #{text}" if caveats.include?(text)
end
copied = caveats.scan(%r{^\s*cp (\S+) }).flatten
raise "the caveats copy #{copied.inspect}, not both samples" unless copied.length == 2
copied.each { |sample| raise "the caveats copy an absent #{sample}" unless File.file?(sample) }
# The production list of installed builds runs as `brew test` would. Its first
# entry is the dependency, and each runtime is one the test knows.
default = formula.ravel_builds
dependency = Formula["linkuistics/taps/ravel"].opt_bin/"ravel"
raise "the dependency is not the first build: #{default.inspect}" unless default.first&.first == dependency
raise "a build has an unknown runtime: #{default.inspect}" unless default.all? { |_, runtime| %w[1.1 2.0].include?(runtime) }
puts "release-formula: this host's installed builds: #{default.map { |ravel, runtime| "#{runtime} #{ravel}" }.join(", ")}"
# The explicit builds replace that list: the TypeScript set against the 1.1
# build and the Rhai set against the 2.0 build.
builds = [[Pathname(ravel_1_1), "1.1"], [Pathname(ravel_2_0), "2.0"]]
formula.define_singleton_method(:ravel_builds) { builds }
formula.instance_variable_set(:@testpath, Pathname(test_path))
Dir.chdir(test_path) { formula.test }
puts "release-formula: production Homebrew install/test passed with independent Ravel 1.1 and 2.0 builds"
