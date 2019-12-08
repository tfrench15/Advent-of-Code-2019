


pub fn part_one(range: (i32, i32)) -> i32 {
    let mut counter = 0;

    for num in range.0..=range.1 {
        let num = num.to_string();
        if is_double(&num) && is_increasing(&num) {
            counter += 1;
        }
    }

    counter
}

fn is_double(num: &str) -> bool {
    let slice: Vec<&str> = num
        .split("")
        .filter(|dig| *dig != "")
        .collect();
    
    for pair in slice.windows(2) {
        if pair[0] == pair[1] {
            return true
        }
    }

    false
}

fn is_increasing(num: &str) -> bool {
    let mut first = num
        .chars()
        .nth(0)
        .unwrap()
        .to_digit(10)
        .unwrap();

    for ch in num.chars().skip(1) {
        let digit = ch.to_digit(10).unwrap();
        if digit < first {
            return false
        } else {
            first = digit;
        }
    }

    true
}
