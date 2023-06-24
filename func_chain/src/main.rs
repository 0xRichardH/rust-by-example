#[derive(Default, Debug)]
struct User {
    email: String,
    name: Option<String>,
    age: Option<u8>,
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
    println!("{:?}", user);
    println!("email: {}", user.email);
    println!("name: {:?}", user.name.unwrap());
    println!("age: {:?}", user.age.unwrap());
}
