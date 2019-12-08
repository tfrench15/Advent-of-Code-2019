mod day_four;
const RANGE: (i32, i32) = (284639, 748759);

fn main() {
    let result = day_four::solution(RANGE);

    println!("{}", result);
}

