use std::io::Read;

pub fn read_file(path: &str) -> String {
    let mut file = std::fs::File::open(path).expect("failed to open file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Unable to read file");
    text
}
