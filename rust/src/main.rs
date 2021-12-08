fn main() {
    println!("Hello, world!");
    test();
}

fn test() {
    let s = String::from("PAYPALISHIRING");
    let n = 3;
    let mut res = String::new();
    if n == 1 {
        return ()
    } else {
        let sb = s.as_bytes();
        if sb.len() < n + 1 {
            return ()
        }

        // init lines
        let mut sv = Vec::new();
        for i in 0..n {
            sv.push(vec![]);
        }

        // 循环
        let mut t = Vec::new();
        let mut i = 0;
        while i < n {
            t.push(i);
            i += 1;
        }
        println!("{:?}", t);

        i = n - 2;
        while i > 0 {
            t.push(i);
            i -= 1;
        }
        // 循环
        println!("{:?}", t);

        // 开始
        let c = sb.len() / t.len();

        // 无须判断的循环
        let mut index = 0;
        for i in 1..c + 1 {
            for j in &t {
                sv[*j].push(sb[index]);
                index += 1;
                println!("{}", index);
            }
        }

        for i in 0..sb.len() - c * t.len() {
            sv[t[i]].push(sb[c * t.len() + i]);
        }

        for i in 0..n {
            res.push_str(&String::from_utf8(sv[i].to_vec()).unwrap());
        }
    }

    println!("{}", "PAYPALISHIRING");
    println!("{}", res);
    println!("{}", "PAHNAPLSIIGYIR");
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
