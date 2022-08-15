
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _query = &args[1];
    let file_path = &args[2];

    println!("{file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("ファイルの中身 {contents}");
}


