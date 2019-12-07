

pub fn part_one(opcode: Vec<i32>) -> Vec<i32> {
    let mut fixed_opcode = opcode.clone();

    for chunk in opcode.chunks(4) {
        match chunk[0] {
            1 => {
                add(&mut fixed_opcode, chunk[1], chunk[2], chunk[3]);
            }, 
            2 => { 
                multiply(&mut fixed_opcode, chunk[1], chunk[2], chunk[3]);
            },
            99 => { 
                break
            },
            _ => panic!("bad opcode!")
        }
    }

    fixed_opcode
}

pub fn part_two(answer: i32) -> Option<i32> { 
    let mut input = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,2,9,23,27,1,6,27,31,1,31,9,35,2,35,10,39,1,5,39,43,2,43,9,47,1,5,47,51,1,51,5,55,1,55,9,59,2,59,13,63,1,63,9,67,1,9,67,71,2,71,10,75,1,75,6,79,2,10,79,83,1,5,83,87,2,87,10,91,1,91,5,95,1,6,95,99,2,99,13,103,1,103,6,107,1,107,5,111,2,6,111,115,1,115,13,119,1,119,2,123,1,5,123,0,99,2,0,14,0];

    for noun in 0..100 {
        for verb in 0..100 {
            match simulate(input, noun, verb, answer) {
                Some(val) => {
                    return Some(val)
                },
                None => {
                    input = reset_input();
                    continue
                },
            }
        }
    }

    None
}

fn simulate(
    mut opcode: Vec<i32>, 
    noun: i32, 
    verb: i32,
    answer: i32
) -> Option<i32> {
    if let Some(elem) = opcode.get_mut(1) {
        *elem = noun;
    }

    if let Some(elem) = opcode.get_mut(2) {
        *elem = verb;
    }

    let result = part_one(opcode);

    if let Some(val) = result.first() {
        if *val == answer {
            return Some(100 * noun + verb)
        }
    }

    None
}

fn add(
    opcode: &mut [i32], 
    pos1: i32, 
    pos2: i32,
    pos3: i32,
) {
    let p1 = opcode.get(pos1 as usize).unwrap();
    let p2 = opcode.get(pos2 as usize).unwrap();

    let sum = *p1 + *p2;

    if let Some(elem) = opcode.get_mut(pos3 as usize) {
        *elem = sum;
    }
}

fn multiply(
    opcode: &mut [i32],
    pos1: i32,
    pos2: i32,
    pos3: i32
) {
    let p1 = opcode.get(pos1 as usize).unwrap();
    let p2 = opcode.get(pos2 as usize).unwrap();

    let product = *p1 * *p2;

    if let Some(elem) = opcode.get_mut(pos3 as usize) {
        *elem = product;
    } 
}

fn reset_input() -> Vec<i32> {
    vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,2,9,23,27,1,6,27,31,1,31,9,35,2,35,10,39,1,5,39,43,2,43,9,47,1,5,47,51,1,51,5,55,1,55,9,59,2,59,13,63,1,63,9,67,1,9,67,71,2,71,10,75,1,75,6,79,2,10,79,83,1,5,83,87,2,87,10,91,1,91,5,95,1,6,95,99,2,99,13,103,1,103,6,107,1,107,5,111,2,6,111,115,1,115,13,119,1,119,2,123,1,5,123,0,99,2,0,14,0]
}