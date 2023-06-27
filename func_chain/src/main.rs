#[derive(Default, Debug)]
pub struct User {
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
        println!("name: {:?}", self.name.unwrap_or_default());
        println!("age: {:?}", self.age.unwrap_or_default());
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
        .with_name(String::from("Richard"))
        .with_age(29);
    user.speak();
}

#[cfg(test)]
mod tests {
    use crate::User;

    #[test]
    fn user_with_default_values() {
        let email = "richard@0xdev.dev";
        let user = User::new(String::from(email));

        assert_eq!(email, user.email);
        assert_eq!(None, user.name);
        assert_eq!(None, user.age);
    }

    #[test]
    fn user_with_name() {
        let email = "test@test.test";
        let name = "Richard";
        let user = User::new(String::from(email)).with_name(String::from(name));

        assert_eq!(email, user.email);
        assert_eq!(name, user.name.unwrap_or_default());
        assert_eq!(None, user.age);
    }

    #[test]
    fn user_with_age() {
        let email = "test@test.test";
        let age = 29;
        let user = User::new(String::from(email)).with_age(age);

        assert_eq!(email, user.email);
        assert_eq!(None, user.name);
        assert_eq!(age, user.age.unwrap_or_default());
    }

    #[test]
    fn user_with_name_and_age() {
        let email = "test@test.test";
        let age = 29;
        let name = "Richard";
        let user = User::new(String::from(email))
            .with_name(String::from(name))
            .with_age(age);

        assert_eq!(email, user.email);
        assert_eq!(name, user.name.unwrap_or_default());
        assert_eq!(age, user.age.unwrap_or_default());
    }
}
