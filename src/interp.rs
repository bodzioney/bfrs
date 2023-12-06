use std::io::{stdin, Read};

const SIZE : usize = 30_000;

pub fn interp(program: &str) {
    let mut tape: [u8; SIZE] = [0; SIZE];
    let mut pointer = 0;

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
            '>' => {
                if pointer == SIZE - 1 {
                    pointer = 0;
                } else {
                    pointer += 1;
                }
            }
            '<' => {
                if pointer == 0 {
                    pointer = SIZE - 1;
                } else {
                    pointer -= 1;
                }
            }
            '+' => tape[pointer] = tape[pointer].wrapping_add(1),
            '-' => tape[pointer] = tape[pointer].wrapping_sub(1),
            '.' => print!("{}", tape[pointer] as char),
            '[' => {
                if tape[pointer] == 0 {
                    i = tbl[i];
                }
            }
            ']' => {
                if tape[pointer] != 0 {
                    i = tbl[i];
                }
            }
            ',' => {
                let mut buf: [u8; 1] = [0];
                stdin().read(&mut buf).unwrap();
                tape[pointer] = buf[0];
            }
            _ => (),
        }
        i += 1;
    }
}
