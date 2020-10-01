pub fn next_phase(input: &String) -> String {
    let mut output = String::with_capacity(input.len());

    // Go through each digit of the input string
    for input_idx in 0..input.len() {
        // Get the pattern for this iteration
        let pattern = pattern_for(input_idx, input.len());

        // Multiply each digit by its corresponding pattern value (and sum all of these)
        let mut value = 0;
        for (input_num_idx, input_num) in input.chars().enumerate() {
            let input_num: isize = input_num.to_string().parse().unwrap();
            let pattern_num = pattern.get(input_num_idx).unwrap();
            value += input_num * pattern_num;
        }

        // Add the rightmost digit of that sum to the output
        let digit = value.to_string().chars().last().unwrap();
        output.push(digit);
    }

    output
}

pub fn pattern_for(input_idx: usize, input_length: usize) -> Vec<isize> {
    let base_pattern = [0, 1, 0, -1];
    let base_pattern_len = 4;

    let mut pattern = Vec::with_capacity(input_length);
    let mut idx = 0;

    loop {
        let num = base_pattern[idx % base_pattern_len];
        let repeat = if idx == 0 {
            input_idx
        } else {
            input_idx + 1
        };

        for _ in 0..repeat {
            if pattern.len() < input_length {
                pattern.push(num);
            } else {
                break;
            }
        }

        if pattern.len() == input_length {
            break;
        }

        idx += 1;
    }

    pattern
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_phase() {
        let input = "12345678";
        assert_eq!(next_phase(&"12345678".to_string()), "48226158".to_string());
        assert_eq!(next_phase(&"48226158".to_string()), "34040438".to_string());
        assert_eq!(next_phase(&"34040438".to_string()), "03415518".to_string());
        assert_eq!(next_phase(&"03415518".to_string()), "01029498".to_string());

        // Large example 1
        let mut input = "80871224585914546619083218645595".to_string();
        for _ in 0..100 {
            input = next_phase(&input);
        }
        assert_eq!(input.chars().take(8).collect::<String>(), "24176176".to_string());

        // Large example 2
        let mut input = "19617804207202209144916044189917".to_string();
        for _ in 0..100 {
            input = next_phase(&input);
        }
        assert_eq!(input.chars().take(8).collect::<String>(), "73745418".to_string());

        // Large example 3
        let mut input = "69317163492948606335995924319873".to_string();
        for _ in 0..100 {
            input = next_phase(&input);
        }
        assert_eq!(input.chars().take(8).collect::<String>(), "52432133".to_string());
    }

    #[test]
    fn test_pattern_for() {
        assert_eq!(pattern_for(0, 8), vec![1, 0, -1, 0, 1, 0, -1, 0]);
        assert_eq!(pattern_for(1, 8), vec![0, 1, 1, 0, 0, -1, -1, 0]);
        assert_eq!(pattern_for(2, 8), vec![0, 0, 1, 1, 1, 0, 0, 0]);
        assert_eq!(pattern_for(3, 8), vec![0, 0, 0, 1, 1, 1, 1, 0]);
    }
}
