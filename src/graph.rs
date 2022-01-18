#[derive(Debug, Clone)]
pub enum EdgeType {
    Digit { digit: u8 },
    Word { id: usize },
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub(crate) edge_type: EdgeType,
    pub(crate) from: usize,
    pub(crate) to: usize,
}

pub struct Graph {
    n: usize,
    adjacency: Vec<Vec<Edge>>
}

impl Graph {
    pub fn new(n: usize, edges: &Vec<Edge>) -> Graph {
        let mut adjacency: Vec<Vec<Edge>> = Vec::new();
        adjacency.resize(n, Vec::new());
        for edge in edges {
            adjacency[edge.from].push(edge.clone());
        }
        Graph {
            n,
            adjacency,
        }
    }
}

pub fn count_paths(graph: &Graph) -> Vec<Vec<EdgeType>> {
    let mut dp: Vec<Vec<Vec<EdgeType>>> = Vec::new();
    dp.resize(graph.n, Vec::new());
    let mut used: Vec<bool> = Vec::new();
    used.resize(graph.n, false);

    fn dfs(v: usize, dp: &mut Vec<Vec<Vec<EdgeType>>>, used: &mut Vec<bool>, graph: &Graph) {
        used[v] = true;
        for edge in &graph.adjacency[v] {
            if edge.to == graph.n {
                dp[v].push(vec![edge.edge_type.clone()]);
                continue;
            }
            if let EdgeType::Digit { .. } = edge.edge_type  {
                if graph.adjacency[v].len() > 1 {
                    continue;
                }
            }
            if used[edge.to] == false {
                dfs(edge.to, dp, used, graph);
            }
            for j in dp[edge.to].clone() {
                if j.is_empty() {
                    continue;
                }
                if let EdgeType::Digit { .. } = edge.edge_type  {
                    if let EdgeType::Digit { .. } = *j.last().unwrap() {
                        continue;
                    }
                }
                let pos = dp[v].len();
                dp[v].push(j.clone());
                dp[v][pos].push(edge.edge_type.clone());
            }
        }
    }

    dfs(0, &mut dp, &mut used, graph);
    dp[0].clone()
}