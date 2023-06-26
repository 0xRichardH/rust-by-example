#[derive(Debug)]
enum TrafficSignal {
    Green,
    Yellow,
    Red,
}

impl TrafficSignal {
    fn next_step(&mut self) {
        use TrafficSignal::*;

        *self = match self {
            Green => Yellow,
            Yellow => Red,
            Red => Green,
        };
    }
}

fn main() {
    let mut signal = TrafficSignal::Green;
    println!("The current signal is {:?}", signal);
    signal.next_step();
    println!("The current signal is {:?}", signal);
    signal.next_step();
    println!("The current signal is {:?}", signal);
}
