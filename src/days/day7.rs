use crate::util::read_lines;

pub fn solution(part: u8) -> u32 {
    let lines = read_lines("./problem_inputs/day4.txt").unwrap();
    match part {
        1 => part1(lines),
        2 => part2(lines),
        _ => 0,
    }
}

fn part1(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    let mut files = bare_tree("files".to_string());
    lines.flatten().for_each(|line| parse(&line, &mut files));
    0
}

fn part2(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    todo!();
}

#[derive(Debug, Clone)]
pub struct Arena<T> {
    nodes: Vec<Node<T>>,
}
#[derive(Debug, Clone, Copy)]
pub struct Node<T> {
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,

    /// The actual data which will be stored within the tree
    pub data: T,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeId {
    index: usize,
}

impl Arena<u32> {
    pub fn new_node(&mut self, data: u32) -> NodeId {
        // Get the next free index
        let next_index = self.nodes.len();
    
        // Push the node into the arena
        self.nodes.push(Node {
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
            data,
        });
    
        // Return the node identifier
        NodeId { index: next_index }
    }
    
}