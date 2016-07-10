// https://rosettacode.org/wiki/Huffman_coding
use std::collections::{BinaryHeap, BTreeMap};
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum NodeData {
    Leaf(char),
    Inner(Box<Node>, Box<Node>),
}

#[derive(PartialEq, Eq)]
struct Node {
    freq: u32,
    data: NodeData,
}

impl PartialOrd<Node> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.freq.cmp(&other.freq).reverse())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq).reverse()
    }
}

#[derive(Copy, Clone, Default)]
struct Encoding {
    bits: u32,
    len: u32,
}

impl Encoding {
    fn add_bit(self, bit: bool) -> Self {
        Encoding {
            bits: self.bits << 1 | bit as u32,
            len: self.len + 1,
        }
    }
    fn get_bit(self, index: u32) -> bool {
        (self.bits >> (self.len - index - 1) & 1) == 1
    }
}

fn build_huffman_tree(char_freq: &BTreeMap<char, u32>) -> Option<Node> {
    let mut queue = BinaryHeap::new();
    for (&symbol, &freq) in char_freq {
        let node = Box::new(Node {
            freq: freq,
            data: NodeData::Leaf(symbol),
        });
        queue.push(node);
    }

    loop {
        match (queue.pop(), queue.pop()) {
            (Some(a), Some(b)) => {
                queue.push(Box::new(Node {
                    freq: a.freq + b.freq,
                    data: NodeData::Inner(a, b),
                }))
            }
            (Some(a), None) => return Some(*a),
            (None, _) => return None,
        }
    }
}

fn fill_encoding_map(encoding_map: &mut BTreeMap<char, Encoding>,
                     node: &Node,
                     encoding: Encoding) {
    match node.data {
        NodeData::Leaf(symbol) => {
            encoding_map.insert(symbol, encoding);
        }
        NodeData::Inner(ref a, ref b) => {
            fill_encoding_map(encoding_map, a, encoding.add_bit(false));
            fill_encoding_map(encoding_map, b, encoding.add_bit(true));
        }
    }
}

struct Huffman {
    root_node: Node,
    encoding_map: BTreeMap<char, Encoding>,
}

impl Huffman {
    fn new(char_freq: &BTreeMap<char, u32>) -> Option<Self> {
        if let Some(root_node) = build_huffman_tree(char_freq) {
            let mut encoding_map = BTreeMap::new();
            fill_encoding_map(&mut encoding_map, &root_node, Encoding::default());
            Some(Huffman {
                root_node: root_node,
                encoding_map: encoding_map,
            })
        } else {
            None
        }
    }

    fn encode_symbol(&self, symbol: char) -> Option<Encoding> {
        match self.encoding_map.get(&symbol) {
            Some(encoding) => Some(*encoding),
            None => None,
        }
    }

    fn encode_string(&self, input: &str) -> Option<Vec<bool>> {
        let mut bits = Vec::new();
        for ch in input.chars() {
            if let Some(encoding) = self.encode_symbol(ch) {
                for i in 0..encoding.len {
                    bits.push(encoding.get_bit(i));
                }
            } else {
                return None;
            }
        }
        Some(bits)
    }

    fn decode_string<'a, I: Iterator<Item = &'a bool>>(&self, mut bits: I) -> String {
        let mut output = String::new();
        let mut node = &self.root_node;
        loop {
            match node.data {
                NodeData::Leaf(symbol) => {
                    output.push(symbol);
                    node = &self.root_node;
                }
                NodeData::Inner(ref left, ref right) => {
                    if let Some(&bit) = bits.next() {
                        node = match bit {
                            false => left,
                            true => right,
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        output
    }
}

fn get_char_freq(input: &str) -> BTreeMap<char, u32> {
    use std::collections::btree_map::Entry::*;
    let mut char_freq = BTreeMap::new();
    for ch in input.chars() {
        match char_freq.entry(ch) {
            Vacant(entry) => {
                entry.insert(1);
            }
            Occupied(entry) => {
                *entry.into_mut() += 1;
            }
        }
    }
    char_freq
}

fn main() {
    let input = "this is an example for huffman encoding";
    println!("original string: {}", input);

    let char_freq = get_char_freq(input);
    let huffman = Huffman::new(&char_freq).unwrap();

    println!("");
    for (symbol, encoding) in &huffman.encoding_map {
        let bit_string = (0..encoding.len)
                             .map(|i| match encoding.get_bit(i) {
                                 false => '0',
                                 true => '1',
                             })
                             .collect::<String>();

        println!("'{}': {}", symbol, bit_string);
    }
    println!("");

    let bits = huffman.encode_string(input).unwrap();

    {
        let bit_string = bits.iter()
                             .map(|b| match *b {
                                 false => '0',
                                 true => '1',
                             })
                             .collect::<String>();
        println!("encoded bits: {}", bit_string);
    }

    let decoded = huffman.decode_string(bits.iter());
    println!("decoded string: {}", decoded);
}
