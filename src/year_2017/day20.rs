use fxhash::FxHashMap;

const SIMULATION_STEPS: usize = 1000;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2017/day_20.txt");
    let mut particles = Universe::new();
    let mut particles_with_collisions = Universe::new();
    particles_with_collisions.collisions_active = true;
    for line in lines.lines() {
        let particle = Particle::from_str(line);
        particles.particles.push(particle);
        particles_with_collisions.particles.push(particle);
    }
    (solve(&mut particles), solve(&mut particles_with_collisions))
}

fn solve(universe: &mut Universe) -> usize {
    for _ in 0..SIMULATION_STEPS {
        universe.step();
    }
    get_closest(&universe.particles)
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
        *self = Self::from_xyz(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

#[derive(Debug, Clone, Copy)]
struct Particle {
    position: Vector3D,
    velocity: Vector3D,
    acceleration: Vector3D,
}

fn parse_vector(s: &str) -> Vector3D {
    let coords = s[3..s.len() - 1]
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    Vector3D::from_xyz(coords[0], coords[1], coords[2])
}

impl Particle {
    fn from_str(line: &str) -> Self {
        let mut iter = line.split(", ");
        let position = parse_vector(iter.next().unwrap());
        let velocity = parse_vector(iter.next().unwrap());
        let acceleration = parse_vector(iter.next().unwrap());
        Self {
            position,
            velocity,
            acceleration,
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
    collisions_active: bool,
}

impl Universe {
    fn new() -> Self {
        Self {
            particles: Vec::new(),
            collisions_active: false,
        }
    }

    fn step(&mut self) {
        for particle in &mut self.particles {
            particle.step();
        }
        if !self.collisions_active {
            return;
        }
        let mut collision_map = FxHashMap::default();
        for (_, p) in self.particles.iter().enumerate() {
            collision_map
                .entry(p.position)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        self.particles.retain(|p| collision_map[&p.position] == 1);
    }
}
