//use bytes:

pub fn solve() {
    let mut input = include_str!("../input/day08")
        .trim()
        .split_whitespace()
        .filter_map(|l| l.parse::<u8>().ok());

    let mut input = parse_node(&mut input);

    let answer = part_one(input);
    println!("part_one={:?}", answer);
}

fn parse_node<I>(i: &mut I) -> Node
where
    I: Iterator<Item = u8>,
{
    let n_children = i.next().unwrap();
    let n_metadata_items = i.next().unwrap();

    let mut node = Node::default();
    for _ in 0..n_children {
        node.children.push(parse_node(i));
    }

    for _ in 0..n_metadata_items {
        let v = i.next().unwrap();
        node.metadata.push(v);
    }

    node
}

#[derive(Default, Debug)]
struct Node {
    metadata: Vec<u8>,
    children: Vec<Node>,
}

impl Node {
    fn sum_of_all_metadata(&self) -> u32 {
        let my_sum: u32 = self.metadata.iter().map(|&n| n as u32).sum();
        let children_sum: u32 = self.children.iter().map(|c| c.sum_of_all_metadata()).sum();
        my_sum + children_sum
    }
}

fn part_one(input: Node) -> u32 {
    input.sum_of_all_metadata()
}

fn part_two() {}
