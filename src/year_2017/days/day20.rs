use itertools::Itertools;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2017/day_20.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> usize {
    let mut particles = Universe::new();
    for line in lines.lines() {
        let particle = Particle::from_str(line);
        particles.particles.push(particle);
    }
    for _ in 0..1000 {
        particles.step();
    }
    get_closest(&particles.particles)
}

fn solve02(lines: &str) -> usize {
    let mut particles = Universe::new();
    for line in lines.lines() {
        let particle = Particle::from_str(line);
        particles.particles.push(particle);
    }
    for _ in 0..1000 {
        particles.step_with_collisions();
    }
    particles.particles.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector3D {
    fn from_xyz(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

impl std::ops::AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::from_xyz(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[derive(Debug, Clone, Copy)]
struct Particle {
    position: Vector3D,
    velocity: Vector3D,
    acceleration: Vector3D,
}

impl Particle {
    fn from_str(line: &str) -> Self {
        let mut iter = line.split(", ");
        let position = iter.next().unwrap();
        let velocity = iter.next().unwrap();
        let acceleration = iter.next().unwrap();
        let position = position[3..position.len() - 1]
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let velocity = velocity[3..velocity.len() - 1]
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let acceleration = acceleration[3..acceleration.len() - 1]
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        Self {
            position: Vector3D::from_xyz(position[0], position[1], position[2]),
            velocity: Vector3D::from_xyz(velocity[0], velocity[1], velocity[2]),
            acceleration: Vector3D::from_xyz(acceleration[0], acceleration[1], acceleration[2]),
        }
    }

    fn get_radius(&self) -> usize {
        self.position.manhattan_distance()
    }

    fn step(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

fn get_closest(particles: &[Particle]) -> usize {
    particles
        .iter()
        .map(Particle::get_radius)
        .enumerate()
        .min_by_key(|(_, r)| *r)
        .unwrap()
        .0
}

#[derive(Debug, Clone)]
struct Universe {
    particles: Vec<Particle>,
}

impl Universe {
    fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    fn step(&mut self) {
        for particle in &mut self.particles {
            particle.step();
        }
    }

    fn step_with_collisions(&mut self) {
        self.step();
        //remove all particles with the same position
        let positions_with_count = self
            .particles
            .iter()
            .map(|p| p.position)
            .counts()
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(p, _)| p)
            .collect::<Vec<_>>();
        self.particles = self
            .particles
            .clone()
            .iter()
            .filter(|p| !positions_with_count.contains(&p.position))
            .map(|p| *p)
            .collect_vec();
    }
}
