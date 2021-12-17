fn main() {
    println!("Hello, world!");
    test();
}

fn test() {
    let s = String::from("abookhere");
    let w = String::from("book");

    let sc = s.chars().into_iter().collect::<Vec<_>>();
    let wc = w.chars().into_iter().collect::<Vec<_>>();

    let t = &sc[1..5];
    if t == wc {
        println!("fk: {}", t[0]);
    }
}