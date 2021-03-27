use std::fs;
use std::fs::File;
use std::io::prelude::*;
use tv_language_trainer::subtitle::*;
use tv_language_trainer::toolbox;

fn _test(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //we split the file into sections
    for s in toolbox::extract_sentences(contents) {
        println!("{}", s);
    }
}
fn _store_to_file(filename: &str, content: Subtitle) {
    let mut file = File::create(filename).unwrap();
    let _x = file.write_all(serde_json::to_string(&content).unwrap().as_bytes());
    // let _x = file.write_all(content.to_string().as_bytes()).unwrap();
}

fn main() {}
