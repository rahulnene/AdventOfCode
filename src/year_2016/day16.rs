pub fn solution() -> (usize, usize) {
    let line = "01111001100111011";
    // let line = "10000";
    (solve01(line), solve02(line))
}

fn solve01(line: &str) -> usize {
    let mut disk = Disk::new(line, 272);
    disk.fill();
    disk.generate_checksum();
    println!("Part 1 checksum: {}",convert_to_string(disk.checksum));
    0
}

fn solve02(line: &str) -> usize {
    let mut disk = Disk::new(line, 35651584);
    disk.fill();
    disk.generate_checksum();
    println!("Part 2 checksum: {}",convert_to_string(disk.checksum));
    0
}

#[derive(Debug)]
struct Disk {
    contents: Vec<bool>,
    max_size: usize,
    checksum: Vec<bool>,
}

impl Disk {
    fn new(str: &str, max_size: usize) -> Self {
        let mut contents = Vec::with_capacity(max_size);
        for c in str.chars() {
            contents.push(c == '1');
        }
        Self {
            contents,
            max_size,
            checksum: Vec::new(),
        }
    }

    fn generate_checksum(&mut self) {
        let mut generated = compute_checksum(&self.contents);
        while generated.len() % 2 == 0 {
            self.checksum = generated.clone();
            generated = compute_checksum(&self.checksum);
        }
        self.checksum = generated;
    }

    fn fill(&mut self) {
        while self.contents.len() < self.max_size {
            let mut new_contents = Vec::new();
            for c in self.contents.iter().rev() {
                new_contents.push(!c);
            }
            self.contents.push(false);
            self.contents.extend(new_contents);
        }
        self.contents.truncate(self.max_size);
    }
}

fn compute_checksum(contents: &Vec<bool>) -> Vec<bool> {
    let mut checksum = Vec::new();
    for i in 0..contents.len() / 2 {
        checksum.push(contents[2 * i] == contents[2 * i + 1]);
    }
    checksum
}

fn convert_to_string(bools: Vec<bool>) -> String {
    let mut string = String::new();
    for b in bools {
        string.push(if b { '1' } else { '0' });
    }
    string
}
