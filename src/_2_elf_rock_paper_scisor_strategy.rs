use crate::utils;
use phf::phf_map;

const MY_SCORES: phf::Map<&'static str, u16> = phf_map! {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
};
const ELF_SCORES: phf::Map<&'static str, u16> = phf_map! {
    "A" => 1,
    "B" => 2,
    "C" => 3,
};

pub fn run() {
    let content = utils::file::read_input_file("_2_elf_rock_paper_scisor_strategy.txt");
    let rounds = get_rounds(&content);
    let score = get_score(&rounds);
    println!("2.1 -- total score is: {score}")
}

fn get_rounds(content: &str) -> Vec<Vec<&str>> {
    return content
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();
}

fn get_score(rounds: &Vec<Vec<&str>>) -> u16 {
    return rounds
        .into_iter()
        .map(get_score_for_one_round)
        .reduce(|acc, item| acc + item)
        .unwrap_or(0);
}

fn get_score_for_one_round(round: &Vec<&str>) -> u16 {
    let elf_letter = round[0];
    let my_letter = round[1];
    let my_score = MY_SCORES.get(my_letter).unwrap_or(&0);

    if ELF_SCORES.get(elf_letter).unwrap_or(&0) == my_score {
        return my_score + 3;
    }
    if (elf_letter == "A" && my_letter == "Y")
        || (elf_letter == "B" && my_letter == "Z")
        || (elf_letter == "C" && my_letter == "X")
    {
        return my_score + 6;
    }

    return my_score + 0;
}
