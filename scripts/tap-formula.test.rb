# Read or exercise a skill formula of the Linkuistics tap with Homebrew itself,
# in a prefix that this script owns. scripts/tap-formula.test.sh calls it.
#
#   brew ruby tap-formula.test.rb source FORMULA
#     prints the formula's name, version, URL and sha256, tab-separated
#   brew ruby tap-formula.test.rb install FORMULA SOURCE KEG TESTPATH
#     runs the formula's install method in SOURCE with its prefix at KEG,
#     then its test method in TESTPATH, then prints its caveats
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

  puts [formula.name, formula.version, formula.stable.url, formula.stable.checksum.hexdigest].join("\t")
when "install"
  raise "expected install FORMULA SOURCE KEG TESTPATH" unless ARGV.length == 3

  source, keg, test_path = ARGV.map { |path| Pathname(path) }
  formula.extend(Homebrew::Assertions)
  # Every keg path of the formula, such as share and libexec, derives from
  # its prefix.
  formula.define_singleton_method(:prefix) { |*_args| keg }
  Dir.chdir(source) { formula.install }
  formula.instance_variable_set(:@testpath, test_path)
  Dir.chdir(test_path) { formula.test }
  puts formula.caveats.to_s
else
  raise "unknown mode #{mode}"
end
