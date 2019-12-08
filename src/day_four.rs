
pub fn part_one(range: (i32, i32)) -> i32 {
    let mut counter = 0;

    for num in range.0..=range.1 {
        let num = num.to_string();
        if has_double(&num) && is_increasing(&num) {
            counter += 1;
        }
    }

    counter
}

pub fn part_two(range: (i32, i32)) -> i32 {
    let mut counter = 0;

    for num in range.0..=range.1 {
        let num = num.to_string();
        if has_double(&num) && is_increasing(&num) {
            counter += 1;
        }
    }

    counter
}

fn has_double(num: &str) -> bool {
    let slice: Vec<&str> = num
        .split("")
        .filter(|dig| *dig != "")
        .collect();

    let mut digit_matches: Vec<i32> = Vec::new();

    let mut in_a_row_count = 1;
    let mut first = slice.first().unwrap();

    for dig in slice.iter().skip(1) {
        if dig == first {
            in_a_row_count += 1;
        } else {
            digit_matches.push(in_a_row_count);
            first = dig;
            in_a_row_count = 1;
        }
    }

    digit_matches.push(in_a_row_count);
    
    digit_matches.contains(&2)
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
