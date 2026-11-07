# Run the install method of a rendered Grove formula with Homebrew itself, in a
# keg that this script owns. scripts/candidate-keg.sh calls it.
#
#   brew ruby candidate-keg.rb FORMULA SOURCE KEG
#
# SOURCE is the extracted archive, the formula's buildpath. KEG is the prefix
# that the install method fills, such as <brew>/Cellar/grove/<version>. The
# script runs no test method, because the candidate keg is a subject of the
# staging trials and not a release check. scripts/release-formula.test.rb runs
# the same install method and then the formula's test.
#
# https://docs.brew.sh/rubydoc/Formula.html#install-instance_method
require "formula"

formula_path, source, keg = ARGV
raise "expected FORMULA SOURCE KEG" unless ARGV.length == 3

# Homebrew's Formulary loads only a formula in a tap, so the script loads the
# file and constructs its class, as scripts/release-formula.test.rb does.
# https://docs.brew.sh/rubydoc/Formula.html#initialize-instance_method
load formula_path
formula = Grove.new("grove", Pathname(formula_path), :stable)
keg = Pathname(keg)
# Every keg path of the formula, such as bin, libexec and share, derives from
# its prefix.
formula.define_singleton_method(:prefix) { |*_args| keg }
formula.instance_variable_set(:@buildpath, Pathname(source))
Dir.chdir(source) { formula.install }
puts "candidate-keg: Homebrew's install method of grove #{formula.version} filled #{keg}"
