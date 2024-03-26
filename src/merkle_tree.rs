use crate::{finite_field::FiniteFieldElement, hashing};

#[derive(Debug)]
struct MerkleTree {
    root: Option<u64>,
    levels: Vec<Vec<u64>>,
}

impl MerkleTree {
    fn new() -> Self {
        MerkleTree {
            root: None,
            levels: Vec::new(),
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
                let hash = hashing::hash(left.wrapping_add(right));

                new_hashes.push(hash); // ab cd ef
            }
            nodes.push(new_hashes.clone()); // [ab cd ef]
            hashes = new_hashes;
        }
        self.root = hashes.pop();
        self.levels = nodes;
    }

    fn root(&self) -> Option<u64> {
        self.root
    }

    fn get_merkle_proof(&self, index: usize) -> Option<Vec<u64>> {
        if index >= self.levels[0].len() {
            return None;
        }
        let mut proof = Vec::new();
        let mut idx = index;
        for level in self.levels.iter() {
            if level.len() == 1 {
                proof.push(level[0]);
                break; // Reached the root node, no need to continue
            }
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            println!("level: {} {}", level.len(), sibling_idx);
            proof.push(level[sibling_idx]);
            idx /= 2;
        }
        Some(proof)
    }
}

#[cfg(test)]
mod tests {
    use crate::{finite_field::FiniteField, hashing::hash};

    use super::*;

    #[test]
    fn empty_tree() {
        let tree = MerkleTree::new();
        assert_eq!(tree.root, None);
        assert_eq!(tree.levels.len(), 0);
    }

    #[test]
    fn build_empty_tree() {
        let mut tree = MerkleTree::new();

        let elements: Vec<FiniteFieldElement> = Vec::new();
        tree.build(&elements);

        assert_eq!(tree.root, None);
        assert_eq!(tree.levels.len(), 1);
        assert_eq!(tree.levels[0].len(), 0);
    }

    #[test]
    fn build_tree_one_element() {
        let mut tree = MerkleTree::new();
        let field = FiniteField::new(13);

        let val: u64 = 3;
        let mut elements: Vec<FiniteFieldElement> = Vec::new();

        elements.push(FiniteFieldElement::new(val, field));
        tree.build(&elements);

        let expected = hash(val);

        assert_eq!(tree.levels.len(), 1);
        assert_eq!(tree.levels[0].len(), 1);

        assert_eq!(tree.root, Some(expected));
        assert_eq!(tree.levels[0][0], expected);
    }

    #[test]
    fn build_tree_two_elements() {
        let mut tree = MerkleTree::new();
        let field = FiniteField::new(13);

        let val1: u64 = 3;
        let val2: u64 = 4;
        let mut elements: Vec<FiniteFieldElement> = Vec::new();

        elements.push(FiniteFieldElement::new(val1, field));
        elements.push(FiniteFieldElement::new(val2, field));
        tree.build(&elements);

        let expected_leaf_1 = hash(val1);
        let expected_leaf_2 = hash(val2);
        let expected_root = hash(expected_leaf_1.wrapping_add(expected_leaf_2));

        assert_eq!(tree.levels.len(), 2);
        assert_eq!(tree.levels[0].len(), 2);
        assert_eq!(tree.levels[1].len(), 1);

        assert_eq!(tree.root, Some(expected_root));
        assert_eq!(tree.levels[0][0], expected_leaf_1);
        assert_eq!(tree.levels[0][1], expected_leaf_2);
    }

    #[test]
    fn build_tree_three_elements() {
        let mut tree = MerkleTree::new();
        let field = FiniteField::new(13);

        let val1: u64 = 3;
        let val2: u64 = 4;
        let val3: u64 = 5;
        let mut elements: Vec<FiniteFieldElement> = Vec::new();

        elements.push(FiniteFieldElement::new(val1, field));
        elements.push(FiniteFieldElement::new(val2, field));
        elements.push(FiniteFieldElement::new(val3, field));
        tree.build(&elements);

        let expected_leaf_1 = hash(val1);
        let expected_leaf_2 = hash(val2);
        let expected_leaf_3 = hash(val3);

        let expected_mid_node1 = hash(expected_leaf_1.wrapping_add(expected_leaf_2));
        let expected_mid_node2 = hash(expected_leaf_3.wrapping_add(expected_leaf_3));

        let expected_root = hash(expected_mid_node1.wrapping_add(expected_mid_node2));

        assert_eq!(tree.levels.len(), 3);
        assert_eq!(tree.levels[0].len(), 3);
        assert_eq!(tree.levels[1].len(), 2);
        assert_eq!(tree.levels[2].len(), 1);

        assert_eq!(tree.root, Some(expected_root));
        assert_eq!(tree.levels[2][0], expected_root);

        assert_eq!(tree.levels[0][0], expected_leaf_1);
        assert_eq!(tree.levels[0][1], expected_leaf_2);
        assert_eq!(tree.levels[0][2], expected_leaf_3);

        assert_eq!(tree.levels[1][0], expected_mid_node1);
        assert_eq!(tree.levels[1][1], expected_mid_node2);
    }

    #[test]
    fn get_merkle_proof_with_three_elements() {
        let mut tree = MerkleTree::new();
        let field = FiniteField::new(13);

        let val1: u64 = 3;
        let val2: u64 = 4;
        let val3: u64 = 5;
        let mut elements: Vec<FiniteFieldElement> = Vec::new();

        elements.push(FiniteFieldElement::new(val1, field));
        elements.push(FiniteFieldElement::new(val2, field));
        elements.push(FiniteFieldElement::new(val3, field));
        tree.build(&elements);

        let expected_leaf_1 = hash(val1);
        let expected_leaf_2 = hash(val2);
        let expected_leaf_3 = hash(val3);

        let expected_mid_node1 = hash(expected_leaf_1.wrapping_add(expected_leaf_2));
        let expected_mid_node2 = hash(expected_leaf_3.wrapping_add(expected_leaf_3));

        let expected_root = hash(expected_mid_node1.wrapping_add(expected_mid_node2));

        let proof = tree.get_merkle_proof(0).unwrap();
        let expected_proof = vec![expected_leaf_2, expected_mid_node2, expected_root];

        for a in proof.iter() {
            println!("Proof {}", a);
        }

        assert_eq!(proof.len(), expected_proof.len());

        for (elem1, elem2) in proof.iter().zip(expected_proof.iter()) {
            assert_eq!(elem1, elem2); // Ensure each pair of corresponding elements is equal
        }
    }
}
