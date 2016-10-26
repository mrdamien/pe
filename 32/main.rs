fn main () {
    let mut products:Vec<u32> = vec![];
    let mut sum = 0;

    for i in 1..9876 {
        for j in 1..987 {
            let p = i * j;

            if is_pandigital_identity (i, j, p) {
                match products.as_slice().binary_search(&p) {
                    Ok(_) => {},
                    Err(n) => {
                        sum += p;
                        products.insert(n, p);
                        println!("{} * {} = {} is a Pandigital Identity", i, j, p);
                    }
                }
            }
        }
    }

    println!("Sum: {}", sum);
}


fn is_pandigital_identity (multiplicand:u32, multiplier:u32, product:u32) -> bool {
    let mut num = multiplicand;
    let mut c = count_digits(multiplicand);
    for _ in 0..count_digits(multiplier) {
        c+= 1;
        if c > 9 {
            return false;
        }
        num *= 10;
    }
    num += multiplier;
    for _ in 0..count_digits(product) {
        c+= 1;
        if c > 9 {
            return false;
        }
        num *= 10;
    }
    num += product;


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
        if mask == mask | (1 << (digit-1)) {
            return false;
        }
        mask = mask | (1 << (digit-1));
    }

    expected_mask == mask
}
