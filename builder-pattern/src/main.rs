// builder pattern: https://www.lurklurk.org/effective-rust/builders.html

#[derive(Default, Debug)]
struct Person {
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    age: Option<u8>,
    dob: Option<chrono::NaiveDate>,
}

struct PersonBuilder(Person);

impl PersonBuilder {
    fn new(email: String) -> Self {
        PersonBuilder(Person {
            email,
            ..Default::default()
        })
    }

    fn first_name(mut self, first_name: String) -> Self {
        self.0.first_name = Some(first_name);
        self
    }

    fn last_name(mut self, last_name: String) -> Self {
        self.0.last_name = Some(last_name);
        self
    }

    fn age(mut self, age: u8) -> Self {
        self.0.age = Some(age);
        self
    }

    fn dob(mut self, dob: chrono::NaiveDate) -> Self {
        self.0.dob = Some(dob);
        self
    }

    fn build(self) -> Person {
        self.0
    }
}

fn main() {
    let person = PersonBuilder::new(String::from("u4kxq@example.com"))
        .first_name(String::from("Richard"))
        .last_name(String::from("H"))
        .age(29)
        .dob(chrono::NaiveDate::from_ymd_opt(1994, 11, 29).unwrap())
        .build();
    println!("{person:?}");
}
