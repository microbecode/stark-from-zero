use crate::{finite_field::FiniteFieldElement, hashing};

// Helper function to find the next power of 2
fn next_power_of_two(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    if n & (n - 1) == 0 {
        return n; // Already a power of 2
    }
    n.next_power_of_two()
}

// Proper two-input hash function for Merkle tree nodes
pub fn hash_two_inputs(a: i128, b: i128) -> i128 {
    let ha = hashing::hash(a);
    let hb = hashing::hash(b);
    // Commutative hashing
    let (lo, hi) = if ha <= hb { (ha, hb) } else { (hb, ha) };
    hashing::hash(lo.wrapping_add(hi))
}

#[derive(Debug)]
pub struct MerkleTree {
    /// Root hash value
    root: Option<i128>,
    /// Nodes of the Merkle tree. Index 0 is leaves
    nodes: Vec<Vec<i128>>,
    /// Padded leaves as field elements (matches leaf hash layer length)
    padded_leaves: Vec<FiniteFieldElement>,
}

impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree {
            root: None,
            nodes: Vec::new(),
            padded_leaves: Vec::new(),
        }
    }

    pub fn build(&mut self, elements: &[FiniteFieldElement]) {
        if elements.is_empty() {
            self.root = None;
            self.nodes = vec![vec![]];
            self.padded_leaves.clear();
            return;
        }

        // Start with hashes of provided elements
        let mut hashes: Vec<i128> = elements.iter().map(|e| e.hash()).collect();

        // Pad hash layer to next power of 2 with literal zero hash values
        let target_size = next_power_of_two(hashes.len());
        while hashes.len() < target_size {
            hashes.push(0);
        }

        // Store padded leaves as field elements of equal length (zeros for padding)
        let mut padded = elements.to_vec();
        while padded.len() < target_size {
            padded.push(FiniteFieldElement::new(0));
        }
        self.padded_leaves = padded;

        let mut nodes = Vec::new();
        nodes.push(hashes.clone());

        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            for chunk in hashes.chunks(2) {
                let hash = hash_two_inputs(chunk[0], chunk[1]);
                new_hashes.push(hash);
            }
            nodes.push(new_hashes.clone());
            hashes = new_hashes;
        }
        self.root = hashes.pop();
        self.nodes = nodes;
    }

    pub fn root(&self) -> Option<i128> {
        self.root
    }

    /// Number of leaf nodes after internal padding to the next power of two
    pub fn leaf_count(&self) -> usize {
        if self.nodes.is_empty() {
            0
        } else {
            self.nodes[0].len()
        }
    }

    /// Access the padded leaves used when building the tree
    pub fn padded_leaves(&self) -> &[FiniteFieldElement] {
        &self.padded_leaves
    }

    pub fn get_merkle_proof(&self, index: usize) -> Option<Vec<i128>> {
        if index >= self.nodes[0].len() {
            return None;
        }
        let mut proof = Vec::new();
        let mut idx = index;
        for level in self.nodes.iter() {
            if level.len() == 1 {
                proof.push(level[0]);
                break; // Reached the root node, no need to continue
            }
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            proof.push(level[sibling_idx]);
            idx /= 2;
        }
        Some(proof)
    }
}

#[cfg(test)]
mod tests {
    use crate::hashing::hash;

    use super::*;

    #[test]
    fn empty_tree() {
        let tree = MerkleTree::new();
        assert_eq!(tree.root, None);
        assert_eq!(tree.nodes.len(), 0);
    }

    #[test]
    fn build_empty_tree() {
        let mut tree = MerkleTree::new();

        let elements: Vec<FiniteFieldElement> = Vec::new();
        tree.build(&elements);

        assert_eq!(tree.root, None);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.nodes[0].len(), 0);
    }

    #[test]
    fn build_tree_one_element() {
        let mut tree = MerkleTree::new();

        let val: i128 = 3;
        let mut elements: Vec<FiniteFieldElement> = Vec::new();

        elements.push(FiniteFieldElement::new(val));
        tree.build(&elements);

        let expected_leaf = hash(val);

        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.nodes[0].len(), 1);

        assert_eq!(tree.root, Some(expected_leaf));
        assert_eq!(tree.nodes[0][0], expected_leaf);
    }

    #[test]
    fn build_tree_two_elements() {
        let mut tree = MerkleTree::new();

        let val1: i128 = 3;
        let val2: i128 = 4;
        let mut elements: Vec<FiniteFieldElement> = Vec::new();

        elements.push(FiniteFieldElement::new(val1));
        elements.push(FiniteFieldElement::new(val2));
        tree.build(&elements);

        let expected_leaf_1 = hash(val1);
        let expected_leaf_2 = hash(val2);
        let expected_root = hash_two_inputs(expected_leaf_1, expected_leaf_2);

        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.nodes[0].len(), 2);
        assert_eq!(tree.nodes[1].len(), 1);

        assert_eq!(tree.root, Some(expected_root));
        assert_eq!(tree.nodes[0][0], expected_leaf_1);
        assert_eq!(tree.nodes[0][1], expected_leaf_2);
    }

    #[test]
    fn build_tree_three_elements() {
        let mut tree = MerkleTree::new();

        let val1: i128 = 3;
        let val2: i128 = 4;
        let val3: i128 = 5;
        let mut elements: Vec<FiniteFieldElement> = Vec::new();

        elements.push(FiniteFieldElement::new(val1));
        elements.push(FiniteFieldElement::new(val2));
        elements.push(FiniteFieldElement::new(val3));
        tree.build(&elements);

        let expected_leaf_1 = hash(val1);
        let expected_leaf_2 = hash(val2);
        let expected_leaf_3 = hash(val3);
        let expected_leaf_4 = 0; // Padding value

        let expected_mid_node1 = hash_two_inputs(expected_leaf_1, expected_leaf_2);
        let expected_mid_node2 = hash_two_inputs(expected_leaf_3, expected_leaf_4);

        let expected_root = hash_two_inputs(expected_mid_node1, expected_mid_node2);

        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.nodes[0].len(), 4);
        assert_eq!(tree.nodes[1].len(), 2);
        assert_eq!(tree.nodes[2].len(), 1);

        assert_eq!(tree.root, Some(expected_root));
        assert_eq!(tree.nodes[2][0], expected_root);

        assert_eq!(tree.nodes[0][0], expected_leaf_1);
        assert_eq!(tree.nodes[0][1], expected_leaf_2);
        assert_eq!(tree.nodes[0][2], expected_leaf_3);
        assert_eq!(tree.nodes[0][3], expected_leaf_4);

        assert_eq!(tree.nodes[1][0], expected_mid_node1);
        assert_eq!(tree.nodes[1][1], expected_mid_node2);
    }

    #[test]
    fn get_merkle_proof_with_three_elements() {
        let mut tree = MerkleTree::new();

        let val1: i128 = 3;
        let val2: i128 = 4;
        let val3: i128 = 5;
        let mut elements: Vec<FiniteFieldElement> = Vec::new();

        elements.push(FiniteFieldElement::new(val1));
        elements.push(FiniteFieldElement::new(val2));
        elements.push(FiniteFieldElement::new(val3));
        tree.build(&elements);

        let expected_leaf_1 = hash(val1);
        let expected_leaf_2 = hash(val2);
        let expected_leaf_3 = hash(val3);
        let expected_leaf_4 = 0; // Padding value

        let expected_mid_node1 = hash_two_inputs(expected_leaf_1, expected_leaf_2);
        let expected_mid_node2 = hash_two_inputs(expected_leaf_3, expected_leaf_4);

        let expected_root = hash_two_inputs(expected_mid_node1, expected_mid_node2);

        // Test proofs for each leaf
        {
            let proof = tree.get_merkle_proof(0).unwrap();
            let expected_proof = vec![expected_leaf_2, expected_mid_node2, expected_root];

            assert_eq!(proof.len(), expected_proof.len());

            for (elem1, elem2) in proof.iter().zip(expected_proof.iter()) {
                assert_eq!(elem1, elem2); // Ensure each pair of corresponding elements is equal
            }
        }
        {
            let proof = tree.get_merkle_proof(1).unwrap();
            let expected_proof = vec![expected_leaf_1, expected_mid_node2, expected_root];

            assert_eq!(proof.len(), expected_proof.len());

            for (elem1, elem2) in proof.iter().zip(expected_proof.iter()) {
                assert_eq!(elem1, elem2); // Ensure each pair of corresponding elements is equal
            }
        }
        {
            let proof = tree.get_merkle_proof(2).unwrap();
            let expected_proof = vec![expected_leaf_4, expected_mid_node1, expected_root];

            assert_eq!(proof.len(), expected_proof.len());

            for (elem1, elem2) in proof.iter().zip(expected_proof.iter()) {
                assert_eq!(elem1, elem2); // Ensure each pair of corresponding elements is equal
            }
        }
        {
            let proof = tree.get_merkle_proof(3).unwrap();
            let expected_proof = vec![expected_leaf_3, expected_mid_node1, expected_root];

            assert_eq!(proof.len(), expected_proof.len());

            for (elem1, elem2) in proof.iter().zip(expected_proof.iter()) {
                assert_eq!(elem1, elem2); // Ensure each pair of corresponding elements is equal
            }
        }
    }

    #[test]
    fn test_padding_to_power_of_two() {
        // Test that tree should properly pad to next power of 2
        // For 5 elements, should pad to 8 (2^3)
        let mut tree = MerkleTree::new();

        let elements: Vec<FiniteFieldElement> =
            (1..=5).map(|i| FiniteFieldElement::new(i)).collect();

        tree.build(&elements);

        // This test will fail until we implement proper power-of-2 padding
        // Should have 4 nodes: leaves (8), level 1 (4), level 2 (2), root (1)
        assert_eq!(tree.nodes.len(), 4);
        assert_eq!(tree.nodes[0].len(), 8); // Should be padded to 8 leaves
        assert_eq!(tree.nodes[1].len(), 4);
        assert_eq!(tree.nodes[2].len(), 2);
        assert_eq!(tree.nodes[3].len(), 1);

        // First 5 leaves should be the original elements
        for i in 0..5 {
            let expected_hash = hash(i as i128 + 1);
            assert_eq!(tree.nodes[0][i], expected_hash);
        }

        // Last 3 leaves should be proper padding (zeros or some default value)
        // This test will fail until we implement proper padding
        for i in 5..8 {
            // Currently duplicates last element, should be padding
            assert_eq!(tree.nodes[0][i], 0); // Should be padding, not duplicate
        }
    }

    #[test]
    fn test_commutative_hashing() {
        // Test that hash(a, b) == hash(b, a) for commutative property
        // The current implementation uses hash(a + b) which is commutative
        // But we need a proper two-input hash function

        // Test the new two-input hash function
        let hash1 = hash_two_inputs(1, 2);
        let hash2 = hash_two_inputs(2, 1);

        // This should pass with our new commutative hash function
        assert_eq!(hash1, hash2);

        // For now, let's test that the current approach works for trees
        let mut tree1 = MerkleTree::new();
        let mut tree2 = MerkleTree::new();

        let elements1: Vec<FiniteFieldElement> =
            vec![FiniteFieldElement::new(1), FiniteFieldElement::new(2)];

        let elements2: Vec<FiniteFieldElement> =
            vec![FiniteFieldElement::new(2), FiniteFieldElement::new(1)];

        tree1.build(&elements1);
        tree2.build(&elements2);

        // This should pass with current implementation
        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_merkle_proof_verification() {
        // Test that we can verify a merkle proof
        let mut tree = MerkleTree::new();

        let elements: Vec<FiniteFieldElement> =
            (1..=4).map(|i| FiniteFieldElement::new(i)).collect();

        tree.build(&elements);

        // Get proof for element at index 0
        let proof = tree.get_merkle_proof(0).unwrap();
        let leaf_hash = hash(1);

        // Verify the proof by reconstructing the root by folding siblings
        // The proof includes the root as the last element; exclude it while folding
        let mut current_hash = leaf_hash;
        for sibling in proof.iter().take(proof.len().saturating_sub(1)) {
            current_hash = hash_two_inputs(current_hash, *sibling);
        }

        assert_eq!(current_hash, tree.root().unwrap());
    }

    #[test]
    fn test_large_tree_current_padding() {
        // Test current padding behavior for larger tree (13 elements -> 16)
        let mut tree = MerkleTree::new();

        let elements: Vec<FiniteFieldElement> =
            (1..=13).map(|i| FiniteFieldElement::new(i)).collect();

        tree.build(&elements);

        // With power-of-2 padding: 13 -> 16 leaves, then 8 -> 4 -> 2 -> 1
        assert_eq!(tree.nodes.len(), 5);
        assert_eq!(tree.nodes[0].len(), 16);
        assert_eq!(tree.nodes[1].len(), 8);
        assert_eq!(tree.nodes[2].len(), 4);
        assert_eq!(tree.nodes[3].len(), 2);
        assert_eq!(tree.nodes[4].len(), 1);

        // First 13 leaves should be original elements
        for i in 0..13 {
            let expected_hash = hash(i as i128 + 1);
            assert_eq!(tree.nodes[0][i], expected_hash);
        }

        // Last 3 leaves should be zero padding
        assert_eq!(tree.nodes[0][13], 0);
        assert_eq!(tree.nodes[0][14], 0);
        assert_eq!(tree.nodes[0][15], 0);
    }

    #[test]
    fn test_large_tree_proper_padding() {
        // Test proper padding for larger tree (13 elements -> 16)
        let mut tree = MerkleTree::new();

        let elements: Vec<FiniteFieldElement> =
            (1..=13).map(|i| FiniteFieldElement::new(i)).collect();

        tree.build(&elements);

        // This test will fail until we implement proper power-of-2 padding
        // Should have 5 nodes: 16, 8, 4, 2, 1
        assert_eq!(tree.nodes.len(), 5);
        assert_eq!(tree.nodes[0].len(), 16); // Should be padded to 16
        assert_eq!(tree.nodes[1].len(), 8);
        assert_eq!(tree.nodes[2].len(), 4);
        assert_eq!(tree.nodes[3].len(), 2);
        assert_eq!(tree.nodes[4].len(), 1);

        // First 13 leaves should be original elements
        for i in 0..13 {
            let expected_hash = hash(i as i128 + 1);
            assert_eq!(tree.nodes[0][i], expected_hash);
        }

        // Last 3 leaves should be proper padding (not duplicates)
        // This test will fail until we implement proper padding
        for i in 13..16 {
            // Currently duplicates last element, should be padding
            assert_eq!(tree.nodes[0][i], 0); // Should be padding, not duplicate
        }
    }
}
