use crate::data::{Chemical, Synthesis};
use std::collections::HashMap;

mod data;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(lines: &Vec<String>) -> usize {
    let mut synthesis = Synthesis::new();
    for line in lines {
        synthesis.add_reaction(line);
    }

    ore_used(&synthesis, 1)
}

fn part_2(lines: &Vec<String>) -> usize {
    let mut synthesis = Synthesis::new();
    for line in lines {
        synthesis.add_reaction(line);
    }

    let ore_available = 1_000_000_000_000;
    let mut guess = 1;
    let mut lower_bound = 1;
    let mut upper_bound = None;

    while upper_bound.is_none() || lower_bound != upper_bound.unwrap() {
        let ore_used = ore_used(&synthesis, guess);

        if ore_used < ore_available {
            lower_bound = guess + 1;
            if let Some(upper) = upper_bound {
                guess = lower_bound + ((upper - lower_bound) / 2);
            } else {
                guess *= 2;
            }
        } else if ore_used == ore_available {
            break;
        } else {
            upper_bound = Some(guess - 1);
            guess = guess - ((guess - lower_bound) / 2);
        }
    }

    guess
}

fn ore_used(synthesis: &Synthesis, fuel_count: usize) -> usize {
    // Initialize counters
    let mut chem_counts: HashMap<String, usize> = HashMap::new();
    let mut total_ore = 0;

    // Start at the end; we want to create 1 FUEL
    let mut out_chems: Vec<Chemical> = Vec::new();
    let fuel = Chemical {
        name: "FUEL".to_string(),
        count: fuel_count,
    };
    out_chems.push(fuel);

    // Iteratively create new chemicals
    while !out_chems.is_empty() {
        // println!("\n\n#######");
        // dbg!(&out_chems);
        let mut next_out_chems: Vec<Chemical> = Vec::new();

        // Create the next list of chemicals that must be "output" from the list of current
        // chemicals we're trying to produce
        for out_chem in out_chems {
            // println!("{} {} is needed", out_chem.count, out_chem.name);

            if out_chem.name == "ORE" {
                total_ore += out_chem.count;
            } else {
                let supply = chem_counts.entry(out_chem.name.clone()).or_insert(0);

                if out_chem.count <= *supply {
                    // Use out_chem.count of the existing supply
                    *supply -= out_chem.count;
                    // println!(
                    //     "    Use {} of our existing {} {}",
                    //     out_chem.count, supply, out_chem.name
                    // );
                    continue;
                } else {
                    let needed = out_chem.count - *supply;
                    let reaction = synthesis.reaction_producing(&out_chem.name);
                    let per_run = reaction.output.count;
                    let reaction_runs = (needed + per_run - 1) / per_run;
                    let will_make = per_run * reaction_runs;
                    let leftover = will_make - needed;

                    // println!(
                    //     "    Using {} {} (have {}); making {} w/ {} leftover via {} runs",
                    //     needed, out_chem.name, supply, will_make, leftover, reaction_runs
                    // );

                    *supply = leftover;

                    for input in reaction.inputs.iter() {
                        let mut input = input.clone();
                        input.count *= reaction_runs;
                        // println!("      => input of {} {}", input.count, input.name);
                        next_out_chems.push(input);
                    }
                }
            }
        }

        // dbg!(&next_outputs);
        // dbg!(&chem_counts);
        out_chems = next_out_chems;
    }

    total_ore
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_1() -> Vec<String> {
        vec![
            "10 ORE => 10 A".to_string(),
            "1 ORE => 1 B".to_string(),
            "7 A, 1 B => 1 C".to_string(),
            "7 A, 1 C => 1 D".to_string(),
            "7 A, 1 D => 1 E".to_string(),
            "7 A, 1 E => 1 FUEL".to_string(),
        ]
    }

    fn example_2() -> Vec<String> {
        vec![
            "9 ORE => 2 A".to_string(),
            "8 ORE => 3 B".to_string(),
            "7 ORE => 5 C".to_string(),
            "3 A, 4 B => 1 AB".to_string(),
            "5 B, 7 C => 1 BC".to_string(),
            "4 C, 1 A => 1 CA".to_string(),
            "2 AB, 3 BC, 4 CA => 1 FUEL".to_string(),
        ]
    }

    fn example_3() -> Vec<String> {
        vec![
            "157 ORE => 5 NZVS".to_string(),
            "165 ORE => 6 DCFZ".to_string(),
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL".to_string(),
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ".to_string(),
            "179 ORE => 7 PSHF".to_string(),
            "177 ORE => 5 HKGWZ".to_string(),
            "7 DCFZ, 7 PSHF => 2 XJWVT".to_string(),
            "165 ORE => 2 GPVTF".to_string(),
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_string(),
        ]
    }

    fn example_4() -> Vec<String> {
        vec![
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG".to_string(),
            "17 NVRVD, 3 JNWZP => 8 VPVL".to_string(),
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL".to_string(),
            "22 VJHF, 37 MNCFX => 5 FWMGM".to_string(),
            "139 ORE => 4 NVRVD".to_string(),
            "144 ORE => 7 JNWZP".to_string(),
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC".to_string(),
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV".to_string(),
            "145 ORE => 6 MNCFX".to_string(),
            "1 NVRVD => 8 CXFTF".to_string(),
            "1 VJHF, 6 MNCFX => 4 RFSQX".to_string(),
            "176 ORE => 6 VJHF".to_string(),
        ]
    }

    fn example_5() -> Vec<String> {
        vec![
            "171 ORE => 8 CNZTR".to_string(),
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL".to_string(),
            "114 ORE => 4 BHXH".to_string(),
            "14 VRPVC => 6 BMBT".to_string(),
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL".to_string(),
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT".to_string(),
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW".to_string(),
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW".to_string(),
            "5 BMBT => 4 WPTQ".to_string(),
            "189 ORE => 9 KTJDG".to_string(),
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP".to_string(),
            "12 VRPVC, 27 CNZTR => 2 XDBXC".to_string(),
            "15 KTJDG, 12 BHXH => 5 XCVML".to_string(),
            "3 BHXH, 2 VRPVC => 7 MZWV".to_string(),
            "121 ORE => 7 VRPVC".to_string(),
            "7 XCVML => 6 RJRHP".to_string(),
            "5 BHXH, 4 VRPVC => 5 LTCX".to_string(),
        ]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&example_1()), 31);
        assert_eq!(part_1(&example_2()), 165);
        assert_eq!(part_1(&example_3()), 13312);
        assert_eq!(part_1(&example_4()), 180697);
        assert_eq!(part_1(&example_5()), 2210736);
    }

    #[test]
    fn test_part_2() {
        // assert_eq!(part_2(&example_3()), 82892753);
        // assert_eq!(part_2(&example_4()), 5586022);
        assert_eq!(part_2(&example_5()), 460664);
    }
}
