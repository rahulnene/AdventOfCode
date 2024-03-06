use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2019/day_8.txt");

pub fn solution() -> ((usize, Duration), (String, Duration)) {
    let image = Image::from_str(LINES.trim(), 25, 6);
    (solve01(&image), solve02(&image))
}

fn solve01(image: &Image) -> (usize, Duration) {
    let now = Instant::now();
    let target = image
        .layers
        .iter()
        .map(|layer| layer.count_pixels())
        .min_by_key(|counts| counts[0])
        .unwrap();
    let result = target[1] * target[2];

    (result, now.elapsed())
}

fn solve02(image: &Image) -> (String, Duration) {
    let now = Instant::now();
    let layer = image.flatten();
    (layer.pprint(), now.elapsed())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Black,
    White,
    Transparent,
}

impl Pixel {
    fn from_char(c: char) -> Pixel {
        match c {
            '0' => Pixel::Black,
            '1' => Pixel::White,
            '2' => Pixel::Transparent,
            _ => panic!("Invalid pixel color"),
        }
    }
}

#[derive(Debug, Clone)]
struct Layer {
    pixels: Vec<Vec<Pixel>>,
}

impl Layer {
    fn from_str(s: &str, width: usize, height: usize) -> Layer {
        let mut pixels = Vec::new();
        for i in 0..height {
            let row = s
                .chars()
                .skip(i * width)
                .take(width)
                .map(Pixel::from_char)
                .collect();
            pixels.push(row);
        }
        Layer { pixels }
    }

    fn count_pixels(&self) -> [usize; 3] {
        let mut counts = [0; 3];
        for row in &self.pixels {
            for p in row {
                match p {
                    Pixel::Black => counts[0] += 1,
                    Pixel::White => counts[1] += 1,
                    Pixel::Transparent => counts[2] += 1,
                }
            }
        }
        counts
    }
    fn pprint(&self) -> String {
        let mut result = "\n".to_string();
        for row in &self.pixels {
            for p in row {
                match p {
                    Pixel::Black => result.push(' '),
                    Pixel::White => result.push('*'),
                    Pixel::Transparent => result.push('?'),
                }
            }
            result.push('\n');
        }
        result
    }
}

#[derive(Debug, Clone)]
struct Image {
    height: usize,
    width: usize,
    layers: Vec<Layer>,
}

impl Image {
    fn from_str(s: &str, width: usize, height: usize) -> Image {
        let layers = s
            .chars()
            .collect::<Vec<_>>()
            .chunks(width * height)
            .map(|chunk| Layer::from_str(&chunk.iter().collect::<String>(), width, height))
            .collect();
        Image {
            height,
            width,
            layers,
        }
    }
    fn flatten(&self) -> Layer {
        let mut pixels = Vec::new();
        for i in 0..self.height {
            let mut row = Vec::new();
            for j in 0..self.width {
                let mut pixel = Pixel::Transparent;
                for layer in &self.layers {
                    match layer.pixels[i][j] {
                        Pixel::Black => {
                            if pixel == Pixel::Transparent {
                                pixel = Pixel::Black;
                            }
                        }
                        Pixel::White => {
                            if pixel == Pixel::Transparent {
                                pixel = Pixel::White;
                            }
                        }
                        Pixel::Transparent => {}
                    }
                }
                row.push(pixel);
            }
            pixels.push(row);
        }
        Layer { pixels }
    }
}
