extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        let n = abs(-3);
        println!("Hello, world! -> {}", n);
    }
}
