fn find_closeset_elements_a(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let mut l = 0;
    let mut r = arr.len() - k;
    let mut start;

    while l < r {
        start = (l + r) / 2;
        if x - arr[start] < arr[start + k] - x {
            r = start;
        } else {
            l = start + 1;
        }
    }

    arr[l..l + k].into()
}

fn main() {
    let arr = vec![-2, -2,-2, 0, 1, 2 , 3,  4, 5, 6];
    let k = 4;
    let x = 10;
    println!("Hello, world!");
    println!("{:?}\t\t{}", arr, x);
    let res = find_closeset_elements_a(arr, k, x);
    println!("{:?}", res);
}
