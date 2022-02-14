fn main() {
    println!("Hello, world!");
    println!("res: {}", is_happy(2));
    let h = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    println!("{}", trap(h));
}


/// 接雨水
/// 从第一个非零数为基准，位置为 a
/// 向右找到大于基准数的位置，例 x， 则池范围为 a 至 x -1
/// 水平面高度为范围内第二高的数字
/// 基准为 x -1，重复上步
fn trap(height: Vec<i32>) -> i32 {
    let mut start = &0;
    let mut pool = vec![];
    let mut res = 0;
    for n in &height {
        if n > start {
            for i in pool {
                res += start - i;
                println!("i: {}", i);
            }
            start = n;
            pool = vec![];
        } else {
            pool.push(*n);
        }
    }

    // 处理余下的 pool
    if pool.len() > 1 {
        pool.reverse();
        pool.push(*start);
        println!("pool: {:?}", pool);
        res += trap(pool);
        res
    } else {
        res
    }
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