fn main() {
    println!("Hello, world!");
    test();
}

fn test() {
    let mut v = vec![0,0,1,1,1,2,2,3,3,4];
    remove_duplicates(&mut v);
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    fn insert_left(nums: &mut Vec<i32>, n: i32) -> usize {
        let mut l = 0;
        let mut r = nums.len();
        let mut mid;

        while l < r {
            mid = (l + r) / 2;
            if n < nums[mid] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        r
    }

    let mut i = 0;
    let mut len = nums.len();

    while i < len {
        let k = insert_left(nums, nums[i]);
        println!("i: {}, k: {}", i, k);
        if k != i + 1 {
            for j in i + 1..k {
                nums.remove(i + 1);
            }
            len -= k - 1 - i;
        }
        println!("v: {:?}", nums);
        i += 1;
    }

    len as i32
}