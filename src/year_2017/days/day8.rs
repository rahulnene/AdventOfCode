use std::ops::Add;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2019/day_8.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut image: Vec<Layer> = Vec::new();
    let width = 25;
    let height = 6;
    let mut split = lines.chars();
    while let Some(_) = split.clone().next() {
        let mut layer = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(split.next().unwrap().to_digit(10).unwrap() as u8);
            }
            layer.push(row);
        }
        image.push(Layer::new_with_data(layer));
    }
    let min_zero_index = image
        .iter()
        .enumerate()
        .min_by_key(|(_, layer)| layer.count(0))
        .unwrap()
        .0;
    let min_layer = &image[min_zero_index];
    min_layer.count(1) * min_layer.count(2)
}

fn solve02(lines: &str) -> usize {
    let mut image: Vec<Layer> = Vec::new();
    let width = 25;
    let height = 6;
    let mut split = lines.chars();
    while let Some(_) = split.clone().next() {
        let mut layer = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(split.next().unwrap().to_digit(10).unwrap() as u8);
            }
            layer.push(row);
        }
        image.push(Layer::new_with_data(layer));
    }
    let mut actual = Layer::new();
    for layer in image {
        actual = actual + layer;
    }
    dbg!(actual);
    0
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Layer {
    data: Vec<Vec<u8>>,
}

impl Layer {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn new_with_data(data: Vec<Vec<u8>>) -> Self {
        Self { data }
    }

    fn new_with_size(width: usize, height: usize) -> Self {
        let mut data = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(0);
            }
            data.push(row);
        }
        Self::new_with_data(data)
    }

    fn from_str(s: &str, width: usize, height: usize) -> Self {
        let mut data = vec![];
        let mut split = s.chars();
        while let Some(_) = split.clone().next() {
            let mut row = vec![];
            for _ in 0..width {
                row.push(split.next().unwrap().to_digit(10).unwrap() as u8);
            }
            data.push(row);
        }
        Self::new_with_data(data)
    }

    fn count(&self, digit: u8) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|&&x| x == digit).count())
            .sum()
    }
}

impl Add for Layer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut data = vec![];
        for (r, row) in self.data.iter().enumerate() {
            let mut new_row = vec![];
            for (c, digit) in row.iter().enumerate() {
                dbg!(digit);
                if *digit == 2 {
                    new_row.push(rhs.data[r][c].clone());
                } else {
                    new_row.push(digit.clone());
                }
            }
            data.push(new_row);
        }
        Self::new_with_data(data)
    }
}
