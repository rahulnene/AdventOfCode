use itertools::Itertools;
use scan_fmt::scan_fmt;
pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2016/day_22.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> usize {
    let node_list = lines
        .lines()
        .skip(2)
        .map(|line| Node::from_str(line))
        .collect::<Vec<_>>();
    node_list
        .iter()
        .tuple_combinations()
        .fold(0, |acc, (a, b)| {
            let a_to_b = check_compat(a, b);
            let b_to_a = check_compat(b, a);
            if a_to_b | b_to_a {
                if a_to_b && b_to_a {
                    acc + 2
                } else {
                    acc + 1
                }
            } else {
                acc
            }
        })
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
    use_percent: usize,
}

impl Node {
    fn new(x: usize, y: usize, size: usize, used: usize, avail: usize, use_percent: usize) -> Self {
        Self {
            x,
            y,
            size,
            used,
            avail,
            use_percent,
        }
    }

    fn from_str(line: &str) -> Self {
        let (x, y, size, used, avail, use_percent) = scan_fmt!(
            line,
            "/dev/grid/node-x{d}-y{d} {d}T {d}T {d}T {d}%",
            usize,
            usize,
            usize,
            usize,
            usize,
            usize
        )
        .expect("Could not parse Node");
        Self::new(x, y, size, used, avail, use_percent)
    }
}

// #[derive(Debug, Clone)]
// struct NodeList {
//     nodes: Vec<Node>,
// }

// impl NodeList {
//     fn new() -> Self {
//         Self { nodes: Vec::new() }
//     }

//     fn add(&mut self, node: Node) {
//         self.nodes.push(node);
//     }
// }

fn check_compat(a: &Node, b: &Node) -> bool {
    a.used != 0 && a.used <= b.avail
}
