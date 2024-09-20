use lalrpop::Configuration;

fn main() {
    Configuration::new()
        .process_file("src/grammar.lalrpop")
        .unwrap();
}
