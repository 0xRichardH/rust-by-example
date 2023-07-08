use std::ops::Deref;

#[derive(Debug)]
struct Amount(f32);

impl Deref for Amount {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn add(a: Amount, b: Amount) -> f32 {
    *a + *b
}

fn main() {
    let a = Amount(3.0);
    let b = Amount(5.0);
    let c = add(a, b);
    println!("a + b = {}", c);
}
