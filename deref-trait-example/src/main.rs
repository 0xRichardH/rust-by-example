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

impl Drop for Amount {
    // this drop function will be called automatically after the instance is out of scope
    fn drop(&mut self) {
        println!("Dropping Amount: {}", self.0);
    }
}

fn main() {
    let a = Amount(3.0);
    let b = Amount(5.0);
    let c = add(a, b);
    println!("a + b = {}", c);
}
