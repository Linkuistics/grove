# Read or exercise a skill formula of the Linkuistics tap with Homebrew itself,
# in a prefix that this script owns. scripts/tap-formula.test.sh calls it.
#
#   brew ruby tap-formula.test.rb source FORMULA
#     prints the formula's name, full package version, URL and sha256,
#     tab-separated. The full package version holds the revision, such as
#     2026-11-06_1, and Homebrew names the keg by it.
#   brew ruby tap-formula.test.rb install FORMULA SOURCE KEG TESTPATH
#     runs the formula's install method in SOURCE, its buildpath, with its
#     prefix at KEG, then its test method in TESTPATH, then prints its caveats
#
# https://docs.brew.sh/rubydoc/Formula.html#test-instance_method
require "formula"
require "formulary"
require "formula_assertions"

mode = ARGV.shift
formula_path = ARGV.shift
raise "expected source FORMULA or install FORMULA SOURCE KEG TESTPATH" if mode.nil? || formula_path.nil?

# Homebrew's Formulary loads only a formula in a tap, so the script loads the
# file and constructs its class, as scripts/release-formula.test.rb does.
# https://docs.brew.sh/rubydoc/Formula.html#initialize-instance_method
formula_path = Pathname(formula_path).expand_path
name = formula_path.basename(".rb").to_s
load formula_path
formula = Object.const_get(Formulary.class_s(name)).new(name, formula_path, :stable)
case mode
when "source"
  raise "expected source FORMULA" unless ARGV.empty?

  puts [formula.name, formula.pkg_version, formula.stable.url, formula.stable.checksum.hexdigest].join("\t")
when "install"
  raise "expected install FORMULA SOURCE KEG TESTPATH" unless ARGV.length == 3

  source, keg, test_path = ARGV.map { |path| Pathname(path) }
  formula.extend(Homebrew::Assertions)
  # Every keg path of the formula, such as share and libexec, derives from
  # its prefix.
  formula.define_singleton_method(:prefix) { |*_args| keg }
  # Homebrew's stage makes SOURCE the formula's buildpath and creates its
  # build HOME, .brew_home, there before the install method runs. A formula
  # that copies its whole source must leave that directory out.
  # https://github.com/Homebrew/brew/blob/7.0.12/Library/Homebrew/formula.rb
  formula.instance_variable_set(:@buildpath, source)
  (source/".brew_home").mkpath
  (source/".brew_home/.gitignore").write "*"
  Dir.chdir(source) { formula.install }
  formula.instance_variable_set(:@testpath, test_path)
  Dir.chdir(test_path) { formula.test }
  puts formula.caveats.to_s
else
  raise "unknown mode #{mode}"
end
