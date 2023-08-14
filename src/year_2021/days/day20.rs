use std::fmt::Debug;

use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_20.txt");
    match part {
        1 => solve(lines, 2),
        2 => solve(lines, 50),
        _ => 1,
    }
}

fn solve(lines: &str, times: usize) -> usize {
    let now = std::time::Instant::now();
    let (algo_str, input) = lines.split_once("\n\n").unwrap();
    let alternate_boundary = algo_str.chars().next().unwrap() == '#';
    let algo = algo_str.chars().map(|f| f == '#').collect_vec();
    let mut image: Image = Image::from_str(input);
    for step in 0..times {
        update(&mut image, &algo, (step % 2 == 0) && alternate_boundary);
    }
    println!("Time taken: {:?}", now.elapsed());
    image.count_light()
}

#[derive(Clone)]
struct Image {
    contents: Vec<Vec<bool>>,
    size: usize,
}

impl Image {
    fn new() -> Self {
        let contents = Vec::new();
        Self { contents, size: 0 }
    }

    fn new_from_size(size: usize, fill: bool) -> Self {
        let contents = vec![vec![fill; size]; size];
        Self { contents, size }
    }

    fn from_str(input: &str) -> Self {
        let mut image = Self::new();
        image.size = input.lines().count();
        for line in input.lines() {
            let mut row: Vec<bool> = Vec::new();
            for c in line.chars() {
                row.push(c == '#');
            }
            image.contents.push(row);
        }
        image
    }

    fn get_safe(&self, row: isize, col: isize, void_fill: bool) -> bool {
        if row < 0 || col < 0 || row >= self.size as isize || col >= self.size as isize {
            return void_fill;
        }
        self.contents[row as usize][col as usize]
    }

    fn sample(&self, x: isize, y: isize, void_fill: bool) -> usize {
        let mut window: Vec<Vec<bool>> = Vec::new();
        for i in x - 1..=x + 1 {
            let mut row: Vec<bool> = Vec::new();
            for j in y - 1..=y + 1 {
                row.push(self.get_safe(i, j, void_fill));
            }
            window.push(row);
        }
        hash(
            &window
                .as_slice()
                .iter()
                .flatten()
                .map(|f| match f {
                    true => "1",
                    false => "0",
                })
                .collect::<String>(),
        )
    }

    fn count_light(&self) -> usize {
        self.contents.iter().flatten().filter(|f| **f).count()
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in 0..self.size {
            for col in 0..self.size {
                if self.contents[row][col] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn hash(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn update(image: &mut Image, algo: &[bool], alternate_boundary: bool) {
    let mut new_image = Image::new_from_size(image.size + 2, alternate_boundary);
    for i in 0..new_image.size {
        for j in 0..new_image.size {
            new_image.contents[i][j] =
                algo[image.sample(i as isize - 1, j as isize - 1, !alternate_boundary)];
        }
    }
    *image = new_image;
}
