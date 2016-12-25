use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

struct Instruction {
    inst: String,
    args: Vec<String>,
}

impl Instruction {
    fn new(inst: &str) -> Self {
        Instruction {
            inst: inst.to_owned(),
            args: Vec::new(),
        }
    }
}

pub fn run(part: i32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => println!("Choose part 1 or 2"),
    }
}

fn part_one() {
    let f = File::open("input/day23.txt").unwrap();
    let reader = BufReader::new(f);
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let inst = line.unwrap();
        let mut spl = inst.split_whitespace();
        let mut instruction = Instruction::new(spl.next().unwrap());

        instruction.args = spl.map(|s| s.to_owned()).collect();
        instructions.push(instruction);
    }

    let max = instructions.len() as i32 - 1;
    let mut ptr = 0;

    let mut register: HashMap<char, i32> = HashMap::new();
    // Use a = 7 for part 1
    register.insert('a', 7);
    register.insert('b', 0);
    register.insert('c', 0);
    register.insert('d', 0);

    loop {
        let inst = instructions[ptr as usize].inst.clone();
        let args = instructions[ptr as usize].args.clone();

        print!("{:>3}:", ptr + 1);

        match inst.as_ref() {
            "cpy" => {
                println!("CPY {:?}", args);
                let mut new_val = 0;
                if let Ok(v) = args[0].parse::<i32>() {
                    new_val = v;
                } else if let Some(other_reg) = args[0].chars().nth(0) {
                    new_val = *register.get(&other_reg).unwrap();
                }
                let reg = args[1].chars().nth(0).unwrap();
                let reg_val = register.entry(reg).or_insert(0);
                *reg_val = new_val;
                ptr += 1;
            },
            "inc" => {
                println!("INC {:?}", args);
                let reg = args[0].chars().nth(0).unwrap();
                let reg_val = register.entry(reg).or_insert(0);
                *reg_val += 1;
                ptr += 1;
            },
            "dec" => {
                println!("DEC {:?}", args);
                let reg = args[0].chars().nth(0).unwrap();
                let reg_val = register.entry(reg).or_insert(0);
                *reg_val -= 1;
                ptr += 1;
            },
            "jnz" => {
                println!("JNZ {:?}", args);
                let mut flag = 0;
                if let Ok(v) = args[0].parse::<i32>() {
                    flag = v;
                } else if let Some(other_reg) = args[0].chars().nth(0) {
                    flag = *register.get(&other_reg).unwrap();
                }

                let mut offset = 0;
                if let Ok(v) = args[1].parse::<i32>() {
                    offset = v;
                } else if let Some(other_reg) = args[1].chars().nth(0) {
                    offset = *register.get(&other_reg).unwrap();
                }


                if flag != 0 {
                    ptr += offset;
                } else {
                    ptr += 1;
                }
            },
            "tgl" => {
                println!("TGL {:?}", args);
                let mut offset = 0;
                if let Ok(v) = args[0].parse::<i32>() {
                    offset = v;
                } else if let Some(other_reg) = args[0].chars().nth(0) {
                    offset = *register.get(&other_reg).unwrap();
                }
                let index = ptr + offset;
                if index <= max {
                    let ref mut other = instructions[index as usize];
                    if other.args.len() == 1 {
                        if other.inst == "inc" {
                            other.inst = "dec".to_owned();
                        } else {
                            other.inst = "inc".to_owned();
                        }
                    } else if other.args.len() == 2 {
                        if other.inst == "jnz" {
                            other.inst = "cpy".to_owned();
                        } else {
                            other.inst = "jnz".to_owned();
                        }
                    }
                }
                ptr += 1;
            },
            _ => panic!("Invalid instruction"),
        }
        println!("{:>6} {:>6} {:>6} {:>6}",
                 register.get(&'a').unwrap(),
                 register.get(&'b').unwrap(),
                 register.get(&'c').unwrap(),
                 register.get(&'d').unwrap(),
                 );

        if ptr > max {
            break;
        }
    }
    println!("Answer: {}", register.get(&'a').unwrap());
}

fn part_two() {
    let mut a = 12;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    b = a; // 1: cpy a b
    b -= 1; // 2: dec b
    loop {
        d = a;
        a = 0;
        loop {
            c = b;
            loop {
                a += 1; // 6
                c -= 1; // 7
                if c == 0 { break; } // 8: jnz c -2
            }
            d -= 1; // 9: dec d
            if d == 0 { break; } // 10: jnz d -5
        }
        b -= 1; // 11: dec b
        c = b; // 12: cpy b c
        d = c; // 13: cpy c d
        loop {
            d -= 1; // 14: dec d
            c += 1; // 15: inc c
            if d == 0 { break; } // 16: jnz d -2
        }
        // c = -16; // 18: cpy -16 c
        if c <= 2 { break; } // 18: jnz 1 c
    }
    c = 1; // 19: cpy 1 c TOGGLED
    c = 79; // 20: cpy 79 c
    loop {
        d = 77; // 21: cpy 77 d TOGGLED
        loop {
            a += 1; // 22: inc a
            d -= 1; // 23: dec d TOGGLED
            if d == 0 { break; } // 24: jnz d -2
        }
        c -= 1; // 25: dec c TOGGLED
        if c == 0 { break; } // 26: jnz c -5
    }
    println!("Answer: {}", a);
}
