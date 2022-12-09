// use std::env;
use std::fs;

pub fn run() {
    const FILE_PATH: &str = "inputs/elf_with_most_calories_01_12_2022.txt";
    match fs::read_to_string(FILE_PATH) {
        Ok(content) => {
            let (max, elf_num) = compute_max_calories(content);
            // let elf_num = idx + 1;
            println!("------------------------------------");
            println!("elf {elf_num} has max cals of {max}cals");
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

fn compute_max_calories(content: String) -> (u32, usize) {
    let mut max = 0;
    let mut total_calories = 0;
    let mut elf_num = 1;
    let mut max_elf_num = 0;
    for line in content.lines() {
        if !line.is_empty() {
            total_calories += line.parse::<u32>().unwrap();
        } else {
            println!("elf {elf_num} has {total_calories}cals.");
            if max < total_calories {
                max = total_calories;
                max_elf_num = elf_num;
            }
            elf_num += 1;
            total_calories = 0;
        }
    }
    return (max, max_elf_num);
}
