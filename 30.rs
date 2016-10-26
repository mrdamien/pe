fn main () {
    let mut sum = 0;
    for i in 2..10000000 {
        if sum_pow(parts(i), 5) == i {
            let m = vec![
                pow (parts(i)[0], 5),
                pow (parts(i)[1], 5),
                pow (parts(i)[2], 5),
                pow (parts(i)[3], 5),
                pow (parts(i)[4], 5),
            ];
            println!("{:?} {:?} {} matches!", parts(i), m, i);
            sum += i;
        }

    }

    println!("{} total", sum);
}

fn sum_pow (parts:Vec<u32>, p:u32) -> u32 {
    let mut sum = 0;
    for i in parts {
        sum = sum + pow(i, p);
    }
    sum
}

fn parts (num:u32) -> Vec<u32> {
    let mut nums:Vec<u32> = vec![];
    let mut b = num;
    while b > 0 || nums.len() < 5 {
        nums.insert(0, (b%10));
        b = b / 10;
    }
    nums
}

fn pow (a:u32, b:u32) -> u32 {
    let mut s = a;
    for _ in 1..b {
        s = s * a;
    }
    s
}
