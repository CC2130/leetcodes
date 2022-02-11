fn main() {
    println!("Hello, world!");
    println!("res: {}", is_happy(2));
}


fn is_happy(n: i32) -> bool {
    let mut n = n;
    let mut t;
    use std::collections::HashSet;

    let mut buffer = HashSet::new();
    while buffer.insert(n) {
        let mut res = 0;
        while n > 0 {
            t = n % 10;
            res += t * t;
            n /= 10;
        }

        println!("num: {}", res);
        if res == 1 {
            return true
        }

        n = res;
    }

    false
}