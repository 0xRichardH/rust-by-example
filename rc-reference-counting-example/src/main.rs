use std::rc::Rc;

#[derive(Debug)]
enum List {
    Node(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Node(5, Rc::new(List::Nil)));
    println!("refercen counter is {}", Rc::strong_count(&a));
    let b = List::Node(6, Rc::clone(&a));
    println!("refercen counter is {}", Rc::strong_count(&a));
    {
        let c = List::Node(7, Rc::clone(&a));
        println!("c: {:?}", c);
        println!("refercen counter is {}", Rc::strong_count(&a));
    }
    println!("refercen counter is {}", Rc::strong_count(&a));
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}
