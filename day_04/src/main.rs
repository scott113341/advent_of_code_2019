fn main() {
    let range_start = 156218;
    let range_end = 652527;

    println!("part_1: {}", part_1(range_start, range_end));
    println!("part_2: {}", part_2(range_start, range_end));
}

// Count numbers in the range that satisfy:
// - It is a six-digit number.
// - The value is within the range given in your puzzle input.
// - Two adjacent digits are the same (like 22 in 122345).
// - Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
fn part_1(range_start: usize, range_end: usize) -> usize {
    let mut count = 0;

    for num in range_start..=range_end {
        let num = format!("{}", num);
        let nums = num.as_bytes();

        if nums.len() != 6 { continue }

        let adjacent = nums[0] == nums[1] ||
            nums[1] == nums[2] ||
            nums[2] == nums[3] ||
            nums[3] == nums[4] ||
            nums[4] == nums[5];
        if !adjacent { continue }

        let increasing = nums[0] <= nums[1] &&
            nums[1] <= nums[2] &&
            nums[2] <= nums[3] &&
            nums[3] <= nums[4] &&
            nums[4] <= nums[5];
        if !increasing { continue }

        count += 1;
    }

    count
}

// All previous criteria, plus:
// - The two adjacent matching digits are not part of a larger group of matching digits
fn part_2(range_start: usize, range_end: usize) -> usize {
    let mut count = 0;

    for num in range_start..=range_end {
        let num = format!("{}", num);
        let nums = num.as_bytes();

        if nums.len() != 6 { continue }

        let increasing = nums[0] <= nums[1] &&
            nums[1] <= nums[2] &&
            nums[2] <= nums[3] &&
            nums[3] <= nums[4] &&
            nums[4] <= nums[5];
        if !increasing { continue }

        let adjacent = (nums[0] == nums[1] && nums[1] != nums[2]) ||
            (nums[1] == nums[2] && nums[0] != nums[1] && nums[2] != nums[3]) ||
            (nums[2] == nums[3] && nums[1] != nums[2] && nums[3] != nums[4]) ||
            (nums[3] == nums[4] && nums[2] != nums[3] && nums[4] != nums[5]) ||
            (nums[4] == nums[5] && nums[3] != nums[4]);
        if !adjacent { continue }

        count += 1;
    }

    count
}
