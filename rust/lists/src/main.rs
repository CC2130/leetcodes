use lists::second::*;

fn main() {
    println!("Hello, world!");
    let mut l = List::new();
    println!("l: {:?}", l);
    l.push(1);
    println!("l: {:?}", l);
    l.push(0);
    println!("l: {:?}", l);
    l.pop();
    println!("l: {:?}", l);
}
