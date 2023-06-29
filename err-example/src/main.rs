use std::{
    fs::File,
    io::{self, Read},
};

// fn main() {
//     let file_name = "hello.txt";
//     let file_result = File::open(file_name);
//     let _ = match file_result {
//         Ok(f) => f,
//         Err(e) => match e.kind() {
//             ErrorKind::NotFound => create_new_file(file_name),
//             other_error => panic!("Error opening file: {}", other_error),
//         },
//     };
// }
//
// fn create_new_file(name: &str) -> File {
//     match File::create(name) {
//         Ok(f) => f,
//         Err(e) => panic!("Error creating file: {}", e),
//     }
// }

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
