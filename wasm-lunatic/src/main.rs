use lunatic::{spawn_link, Mailbox};

#[lunatic::main]
fn main(_: Mailbox<()>) {
    let name = "Richard".to_string();
    let child = spawn_link!(@task |name| {
        println!("Hello, world!");
        format!("I am {}", name)
    });

    let result = child.result();
    println!("{result}");
}
