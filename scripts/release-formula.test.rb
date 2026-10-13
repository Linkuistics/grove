# Exercise the production formula with Homebrew itself in an owned prefix.
# Adapted from Ravel's installed-formula seam; no mocked Formula DSL.
# https://docs.brew.sh/rubydoc/Formula.html#test-instance_method
require "formula"
require "formula_assertions"

formula_path, source, prefix_path, test_path = ARGV
raise "expected FORMULA SOURCE PREFIX TESTPATH" unless ARGV.length == 4
prefix_path = Pathname(prefix_path)
load formula_path
# https://docs.brew.sh/rubydoc/Formula.html#initialize-instance_method
formula = Grove.new("grove", Pathname(formula_path), :stable)
formula.extend(Homebrew::Assertions)
formula.define_singleton_method(:prefix) { |*_args| prefix_path }
expected = (["bin", "libexec"] + Dir.glob("{bin,libexec}/**/*", base: source)).sort
Dir.chdir(source) { formula.install }
actual = Dir.glob("**/*", base: prefix_path).sort
raise "formula installation differs from archive: #{actual.inspect}" unless actual == expected
formula.instance_variable_set(:@testpath, Pathname(test_path))
Dir.chdir(test_path) { formula.test }
puts "release-formula: production Homebrew install/test passed with independent installed Ravel"
