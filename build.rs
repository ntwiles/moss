use lalrpop::Configuration;

fn main() {
    // Configuration::new()
    //     .set_out_dir("dir")
    //     .process_file("src/grammar.lalrpop")
    //     .unwrap();
    lalrpop::process_root().unwrap();
}
