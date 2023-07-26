use std::collections::BinaryHeap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Task {
    A,
    B,
    C,
}

fn main() {
    let mut queue = BinaryHeap::new();

    queue.push((0, Task::A));
    queue.push((0, Task::C));
    queue.push((1, Task::B));
    queue.push((1, Task::C));
    queue.push((2, Task::C));
    queue.push((0, Task::B));

    while let Some((p, task)) = queue.pop() {
        println!("{} {:?}", p, task);
    }
}
