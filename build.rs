use lalrpop::Configuration;

fn main() {
    Configuration::new()
        .use_cargo_dir_conventions()
        .process_file("src/grammar.lalrpop")
        .expect("Failed to process LALRPOP grammar");
}
