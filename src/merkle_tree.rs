use crate::{finite_field::FiniteFieldElement, hashing};

#[derive(Debug)]
struct MerkleTree {
    root: Option<u64>,
    nodes: Vec<Vec<u64>>,
}

impl MerkleTree {
    fn new() -> Self {
        MerkleTree {
            root: None,
            nodes: Vec::new(),
        }
    }

    fn build(&mut self, elements: &[FiniteFieldElement]) {
        let mut hashes: Vec<u64> = elements.iter().map(|e| e.hash()).collect();
        let mut nodes = Vec::new();
        nodes.push(hashes.clone());

        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            for chunk in hashes.chunks(2) {
                let (left, right) = match chunk.len() {
                    1 => (chunk[0], chunk[0]),
                    2 => (chunk[0], chunk[1]),
                    _ => unreachable!(),
                };
                let hash = hashing::hash(&format!("{}{}", left.to_string(), right.to_string()));

                new_hashes.push(hash);
            }
            nodes.push(new_hashes.clone());
            hashes = new_hashes;
        }
        self.root = hashes.pop();
        self.nodes = nodes;
    }

    fn root(&self) -> Option<u64> {
        self.root
    }

    fn get_merkle_proof(&self, index: usize) -> Option<Vec<u64>> {
        if index >= self.nodes[0].len() {
            return None;
        }
        let mut proof = Vec::new();
        let mut idx = index;
        for level in &self.nodes {
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            proof.push(level[sibling_idx]);
            idx /= 2;
        }
        Some(proof)
    }
}
