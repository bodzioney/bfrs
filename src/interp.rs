use std::io::{stdin, Read};

pub fn interp(program: &str) {
    let mut data: [u8; 30000] = [0; 30000];
    let mut ind = 0;
    
    let mut tbl = vec![0; program.len()];

    // Populate jump table
    let mut stack: Vec<usize> = vec![];
    for (i, c) in program.chars().enumerate() {
        if c == '[' {
            stack.push(i);
        } else if c == ']' && stack.len() != 0 {
            let par = stack.pop().unwrap();

            tbl[i] = par;
            tbl[par] = i;
        }
    }

    // Interpret program
    let instructions = program.as_bytes();
    let mut i = 0;
    while i < program.len() {
         match instructions[i] as char {
            '>' => ind += 1,
            '<' => ind -= 1,
            '+' => data[ind] = data[ind].wrapping_add(1),
            '-' => data[ind] = data[ind].wrapping_sub(1),
            '.' => print!("{}", data[ind] as char),
            '[' => {
                if data[ind] == 0 {
                    i = tbl[i]
                }
            }
            ']' => {
                if data[ind] != 0 {
                    i = tbl[i]
                }
            }
            ',' => {
                let mut buf: [u8; 1] = [0];
                stdin().read(&mut buf).unwrap();
            }
            _ => (),
        }
        i += 1;
    }
}
