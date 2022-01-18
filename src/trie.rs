#[derive(Default, Debug)]
pub struct TrieNode {
    pub next: [Option<usize>; 10],
    pub terminals: Vec<usize>,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {
            next: [Option::None; 10],
            terminals: vec![],
        }
    }
}

#[derive(Default, Debug)]
pub struct Trie {
    pub root: usize,
    pub nodes: Vec<TrieNode>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: 0,
            nodes: vec![TrieNode::new()],
        }
    }

    pub fn add(&mut self, s: &Vec<u8>, i: usize) {
        let mut v = 0;
        for &c in s {
            if self.nodes[v].next[c as usize] == Option::None {
                self.nodes[v].next[c as usize] = Some(self.nodes.len());
                self.nodes.push(TrieNode::new());
            }
            v = self.nodes[v].next[c as usize].unwrap();
        }
        self.nodes[v].terminals.push(i);
    }
}