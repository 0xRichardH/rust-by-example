#[derive(Debug)]
struct Price(f32);

fn add(Price(a): &Price, Price(b): &Price) -> f32 {
    a + b
}

fn main() {
    let a = Price(1.0);
    let b = Price(2.0);
    let c = add(&a, &b);
    println!("{:?} + {:?} = {}", a, b, c);
}
