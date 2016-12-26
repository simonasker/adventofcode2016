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

fn execute_instructions(input: i32, instructions: &Vec<Instruction>) -> Vec<i32> {
    let max = instructions.len() as i32 - 1;
    let mut ptr = 0;

    let mut output: Vec<i32> = Vec::new();

    let mut register: HashMap<char, i32> = HashMap::new();
    register.insert('a', input);
    register.insert('b', 0);
    register.insert('c', 0);
    register.insert('d', 0);

    loop {
        let inst = instructions[ptr as usize].inst.clone();
        let args = instructions[ptr as usize].args.clone();

        // print!("{:>3}:", ptr + 1);

        match inst.as_ref() {
            "cpy" => {
                // println!("CPY {:?}", args);
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
                // println!("INC {:?}", args);
                let reg = args[0].chars().nth(0).unwrap();
                let reg_val = register.entry(reg).or_insert(0);
                *reg_val += 1;
                ptr += 1;
            },
            "dec" => {
                // println!("DEC {:?}", args);
                let reg = args[0].chars().nth(0).unwrap();
                let reg_val = register.entry(reg).or_insert(0);
                *reg_val -= 1;
                ptr += 1;
            },
            "jnz" => {
                // println!("JNZ {:?}", args);
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
            "out" => {
                // println!("DEC {:?}", args);
                let mut val = 0;
                if let Ok(v) = args[0].parse::<i32>() {
                    val = v;
                } else if let Some(other_reg) = args[0].chars().nth(0) {
                    val = *register.get(&other_reg).unwrap();
                }

                if let Some(last) = output.last() {
                    if val == *last {
                        break;
                    }
                }
                output.push(val);
                ptr += 1;
            },
            _ => panic!("Invalid instruction"),
        }
        // println!("{:>6} {:>6} {:>6} {:>6}",
        //          register.get(&'a').unwrap(),
        //          register.get(&'b').unwrap(),
        //          register.get(&'c').unwrap(),
        //          register.get(&'d').unwrap(),
        //          );

        if ptr > max || output.len() > 100 {
            break;
        }
    }

    output
}

fn part_one() {
    let f = File::open("input/day25.txt").unwrap();
    let reader = BufReader::new(f);
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let inst = line.unwrap();
        let mut spl = inst.split_whitespace();
        let mut instruction = Instruction::new(spl.next().unwrap());

        instruction.args = spl.map(|s| s.to_owned()).collect();
        instructions.push(instruction);
    }

    for i in 0.. {
        let output = execute_instructions(i, &instructions);
        if output.len() > 100 {
            println!("{}: {:?}", i, output);
        }
    }
}

fn part_two() {
    println!("Not yet implemented");
}
