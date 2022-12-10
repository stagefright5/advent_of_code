use std::fs;

pub fn run() {
    const FILE_PATH: &str = "inputs/elf_with_most_calories_01_12_2022.txt";
    match fs::read_to_string(FILE_PATH) {
        Ok(content) => {
            let (max, elf_num) = compute_max_calories(content.clone());
            println!("------------------------------------");
            println!("elf {elf_num} has max cals of {max}cals");
            let sum_of_max_3 = get_sum_of_max_3_calories(content);
            println!("------------------------------------");
            println!("Sum of max 3 cals is {sum_of_max_3}");
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
            // println!("elf {elf_num} has {total_calories}cals.");
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

fn get_sum_of_max_3_calories(content: String) -> u32 {
    let mut total_calories: Vec<u32> = Vec::new();
    let mut elf_cals = 0;
    for line in content.lines() {
        if !line.is_empty() {
            elf_cals += line.parse::<u32>().unwrap();
        } else {
            total_calories.push(elf_cals);
            elf_cals = 0;
        }
    }
    total_calories.sort_unstable_by(|a, b| (*b).cmp(a));
    return total_calories[..=2]
        .to_vec()
        .into_iter()
        .reduce(|acc, item| acc + item)
        .unwrap();
}
