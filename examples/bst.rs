use range_filters::binary_search_tree::BinarySearchTreeGroup;

fn main() {
    let bst = BinarySearchTreeGroup::new_with_keys(&[1, 2, 3, 20, 30, 4, 5, 6, 7]);
    bst.pretty_print();

    let mut bst2 = BinarySearchTreeGroup::new();
    bst2.pretty_print();
    for &key in &[50, 25, 75, 12, 37, 62, 87] {
        bst2.insert(key);
    }
    bst2.pretty_print();

    // unbalanced tree
    let mut bst3 = BinarySearchTreeGroup::new();
    for &key in &[1, 2, 3, 4, 5] {
        bst3.insert(key);
    }
    bst3.pretty_print();
}