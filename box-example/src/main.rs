#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Nil,
}

fn main() {
    let mut list = List::Node(0, Box::new(List::Nil));
    for ele in 1..=3 {
        list = List::Node(ele, Box::new(list));
    }
    println!("{list:?}");
}
