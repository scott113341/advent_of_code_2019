use crate::data::{Image, count_digits};

mod data;

fn main() {
    let input = include_str!("input.txt").trim();
    let image = Image::new(input, 25, 6);

    println!("part_1: {}", part_1(&image));
    print!("part_2:");
    part_2(&image);
    print!("\n");
}

// Find the layer that contains the fewest 0 digits. On that layer, what is the number of 1 digits
// multiplied by the number of 2 digits?
fn part_1(image: &Image) -> usize {
    let layer = image.layers
        .iter()
        .min_by_key(|layer| count_digits(layer, 0))
        .unwrap();

    let count_1_digits = count_digits(layer, 1);
    let count_2_digits = count_digits(layer, 2);

    count_1_digits * count_2_digits
}

// 0 is black, 1 is white, and 2 is transparent.
fn part_2(image: &Image) {
    let Image { width, height, .. } = image;
    let pixel_count = width * height;

    for idx in 0..pixel_count {
        let color = image.layers
            .iter()
            .find_map(|l| if l[idx] < 2 { Some(l[idx]) } else { None })
            .unwrap();

        if idx % width == 0 {
            print!("\n");
        }

        if color == 0 {
            print!("█");
        } else {
            print!("░");
        }
    }
}
