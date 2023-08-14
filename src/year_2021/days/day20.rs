use itertools::Itertools;
use std::fmt::Debug;

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
    let algo = algo_str.chars().map(|f| f == '#').collect::<Vec<_>>();
    let mut image = Image::from_str(input);
    for step in 0..times {
        update(
            &mut image,
            &algo,
            step % 2 == 0 && algo_str.starts_with('#'),
        );
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
    fn new_from_size(size: usize, fill: bool) -> Self {
        let contents = vec![vec![fill; size]; size];
        Self { contents, size }
    }

    fn from_str(input: &str) -> Self {
        let lines = input.lines();
        let contents = lines
            .clone()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Self {
            size: lines.count(),
            contents,
        }
    }

    fn get_safe(&self, row: isize, col: isize, void_fill: bool) -> bool {
        *self
            .contents
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .unwrap_or(&void_fill)
    }

    fn sample(&self, x: isize, y: isize, void_fill: bool) -> usize {
        let window = (x - 1..=x + 1)
            .flat_map(|i| (y - 1..=y + 1).map(move |j| self.get_safe(i, j, void_fill)))
            .collect::<Vec<bool>>();
        hash(
            &window
                .iter()
                .map(|f| if *f { "1" } else { "0" })
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
    for (i, row) in new_image.contents.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = algo[image.sample(i as isize - 1, j as isize - 1, !alternate_boundary)];
        }
    }
    *image = new_image;
}
