fn main () {
    // let n = 3;
    // let num = 987654321;
    // let result = take_digit(num, n);
    //println!("The {}-th digit of {} is {}. Whats left is: {}", n, num, result.0, result.1);
    let mut products:Vec<u32> = vec![];
    let mut sum = 0;

    for i in 1..9876 {
        for j in 1..9876 {
            if has_repeat(i) {
                continue;
            }
            if has_repeat(j) {
                continue;
            }
            let p = i * j;
            if has_repeat(p) {
                continue;
            }

            if count_digits(i) + count_digits(j) + count_digits(p) > 9 {
                continue;
            }


            if is_pandigital_identity (i, j, p) {
                match products.as_slice().binary_search(&p) {
                    Ok(n) => {
                        // already found
                    },
                    Err(n) => {
                        sum += p;
                        products.insert(n, p);
                        println!("{} * {} = {} is a Pandigital Identity", i, j, p);
                    }
                }
                // println!("{} * {} = {} is a Pandigital Identity", i, j, p);
            }
        }
        //println!("{}", i);
    }

    println!("Sum: {}", sum);
}

fn is_pandigital_identity (multiplicand:u32, multiplier:u32, product:u32) -> bool {
    let mut num = multiplicand;
    for _ in 0..count_digits(multiplier) {
        num *= 10;
    }
    num += multiplier;
    for _ in 0..count_digits(product) {
        num *= 10;
    }
    num += product;

    //println!("{} * {} = {} -> {}", multiplicand, multiplier, product, num);

    is_pandigital(num)
}

fn count_digits (num:u32) -> i32 {
    let mut n = 0;
    let mut num = num;
    while num > 0 {
        n += 1;
        num /= 10;
    }
    n
}

fn is_pandigital (num:u32) -> bool {
    let mut mask:i32 = 0;
    let mut num = num;
    let expected_mask:i32 = 0b111111111;

    while num > 0 {
        let digit = num % 10;
        num /= 10;
        if digit == 0 {
            continue;
        }
        mask = mask | (1 << (digit-1));
    }

    expected_mask == mask
}

fn has_repeat (num:u32) -> bool {
    let mut mask = 0;
    let mut num = num;
    while num > 0 {
        let digit = num % 10;
        num /= 10;
        let bit = (1 << digit);
        if mask | bit == mask {
            return true;
        }
        mask = mask | bit;
    }
    false
}
