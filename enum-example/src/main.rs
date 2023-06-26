struct Content {
    message: String,
}

enum HttpStatus {
    Success(Content),
    NotFound,
    InternalServerError,
}

impl HttpStatus {
    fn render(self) {
        use HttpStatus::*;
        match self {
            Success(content) => println!("{}", content.message),
            NotFound => println!("404"),
            InternalServerError => println!("500"),
        }
    }
}

fn main() {
    HttpStatus::Success(Content {
        message: "Hello World".to_string(),
    })
    .render();

    HttpStatus::NotFound.render();
    HttpStatus::InternalServerError.render();
}
