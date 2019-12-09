pub struct Image {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}

pub type Layer = Vec<Pixel>;
pub type Pixel = usize;

impl Image {
    pub fn new(data: &str, width: usize, height: usize) -> Image {
        let pixel_count = width * height;
        let mut layers = vec![];
        let mut chars: Vec<char> = data.chars().collect();

        while chars.len() > 0 {
            let mut layer = Vec::with_capacity(pixel_count);
            while layer.len() < pixel_count {
                layer.push(chars.remove(0) as usize - '0' as usize);
            }
            layers.push(layer);
        }

        Image {
            width,
            height,
            layers,
        }
    }
}

pub fn count_digits(layer: &Layer, digit: usize) -> usize {
    layer
        .iter()
        .filter(|&&d| d == digit)
        .count()
}
