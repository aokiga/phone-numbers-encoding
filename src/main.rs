mod io;
mod graph;
mod trie;

use crate::graph::{count_paths, Edge, EdgeType, Graph};
use crate::io::write_file;
use crate::trie::Trie;

fn char_to_num(c: char) -> u8 {
    match c {
        'e' => 0u8,
        'j' | 'n' | 'q' => 1u8,
        'r' | 'w' | 'x' => 2u8,
        'd' | 's' | 'y' => 3u8,
        'f' | 't' => 4u8,
        'a' | 'm' => 5u8,
        'c' | 'i' | 'v' => 6u8,
        'b' | 'k' | 'u' => 7u8,
        'l' | 'o' | 'p' => 8u8,
        'g' | 'h' | 'z' => 9u8,
        _ => panic!("unexpected symbol")
    }
}

fn parse_word(s: String) -> Vec<u8> {
    s.to_ascii_lowercase()
        .chars()
        .filter(|c| (*c) != '-' && (*c) != '\"')
        .map(char_to_num)
        .collect()
}

fn parse_phone_number(phone_number: &String) -> Vec<u8> {
    phone_number
        .chars()
        .filter(|&s| s != '-' && s != '/')
        .map(|s| s as u8 - '0' as u8)
        .collect()
}

fn edge_type_to_string(words: &Vec<String>, phone: &Vec<u8>, edge_type: EdgeType) -> String {
    match edge_type {
        EdgeType::Digit { digit } => digit.to_string(),
        EdgeType::Word { id } => words[id].clone()
    }
}

fn request(words: &Vec<String>, trie: &Trie, phone_not_parsed: &String) -> Vec<String> {
    let phone = parse_phone_number(phone_not_parsed);
    let mut edges: Vec<Edge> = Vec::new();
    for prefix in 0..phone.len() {
        let mut v: usize = trie.root;
        let mut flag = true;
        for r in prefix..phone.len() {
            if trie.nodes[v].next[phone[r] as usize] == Option::None {
                break;
            }
            v = trie.nodes[v].next[phone[r] as usize].unwrap();
            for terminal in &trie.nodes[v].terminals {
                flag = false;
                edges.push(Edge {
                    edge_type: EdgeType::Word { id: *terminal },
                    from: prefix,
                    to: r + 1,
                })
            }
        }
        if flag {
            edges.push(Edge {
                edge_type: EdgeType::Digit { digit: phone[prefix] },
                from: prefix,
                to: prefix + 1,
            })
        }
    }
    let graph = Graph::new(phone.len(), &edges);
    count_paths(&graph).into_iter()
        .map(|path: Vec<EdgeType>| format!("{}: {}", phone_not_parsed, path.into_iter().rev()
            .map(|edge_type| {
                match edge_type {
                    EdgeType::Digit { digit } => digit.to_string(),
                    EdgeType::Word { id } => words[id].clone()
                }
            }).collect::<Vec<String>>().join(" "))
        ).collect()
}

fn create_trie(words: &Vec<String>) -> Trie {
    let num_words: Vec<Vec<u8>> = words.clone().into_iter().map(parse_word).collect();
    let mut trie = Trie::new();
    for i in 0..num_words.len() {
        trie.add(&num_words[i], i);
    }
    trie
}

fn solve(words: &Vec<String>, phones: &Vec<String>) -> Vec<String> {
    let trie = create_trie(words);
    phones.into_iter()
        .map(|phone| request(words, &trie, phone))
        .flatten()
        .collect()
}

fn main() {
    let words: Vec<String> = crate::io::read_file("dictionary.txt").to_vec();
    let phones: Vec<String> = crate::io::read_file("input.txt");
    let res = solve(&words, &phones).join("\n").to_string();
    write_file("output.txt", &res);
}
