fn main() {
    println!("Hello, world!");
    println!("res: {}", is_happy(2));
}


/// 接雨水
/// 从第一个非零数为基准，位置为 a
/// 向右找到小于基准数的位置，例 x， 则池范围为 a 至 x -1
/// 基准为 x -1，重复上步
fn trap(height: Vec<i32>) -> i32 {
    0
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