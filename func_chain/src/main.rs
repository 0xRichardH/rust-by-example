#[derive(Default, Debug)]
struct User {
    email: String,
    name: Option<String>,
    age: Option<u8>,
}

trait Human {
    fn speak(self);
}

impl Human for User {
    fn speak(self) {
        println!("{:?}", self);
        println!("email: {}", self.email);
        println!("name: {:?}", self.name.unwrap());
        println!("age: {:?}", self.age.unwrap());
    }
}

impl User {
    fn new(email: String) -> Self {
        Self {
            email,
            ..Default::default()
        }
    }

    fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn with_age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }
}

fn main() {
    let user = User::new("richard@0xdev.dev".to_string())
        .with_name("richard".to_string())
        .with_age(29);
    user.speak();
}
