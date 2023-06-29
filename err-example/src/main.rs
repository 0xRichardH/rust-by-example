use std::{fs::File, io::ErrorKind};

fn main() {
    let file_name = "hello.txt";
    let file_result = File::open(file_name);
    let _ = match file_result {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => create_new_file(file_name),
            other_error => panic!("Error opening file: {}", other_error),
        },
    };
}

fn create_new_file(name: &str) -> File {
    match File::create(name) {
        Ok(f) => f,
        Err(e) => panic!("Error creating file: {}", e),
    }
}
