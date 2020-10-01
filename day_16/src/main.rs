use crate::data::next_phase;

mod data;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .to_string();

    // println!("part_1: {}", part_1(input.clone()));
    println!("part_2: {}", part_2(input.clone()));
}

fn part_1(mut input: String) -> String {
    for _ in 0..100 {
        input = next_phase(&input);
    }
    input.chars().take(8).collect::<String>()
}

fn part_2(mut input: String) -> String {
    let offset = input[0..7].to_string().parse::<usize>().unwrap();
    let idx_start = offset - 1;
    let idx_end = idx_start + 8;

    input = input.repeat(10_000);

    for _ in 0..100 {
        input = next_phase(&input);
        println!("{}", &input);
    }

    input[idx_start..idx_end].to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {

    }

    #[test]
    fn test_part_2() {
    }
}
