

pub fn solve(opcode: Vec<i32>) -> Vec<i32> {
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