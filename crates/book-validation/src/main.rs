fn main() {
    let output = book_validation::cli::run_from(std::env::args_os());
    print!("{}", output.stdout);
    eprint!("{}", output.stderr);
    std::process::exit(output.exit);
}
