use range_filters::x_fast_trie::XFastTrie;

fn main() {
    let mut trie = XFastTrie::new(8);
    
    let keys = vec![10, 5, 15, 3, 12];
    
    for key in &keys {
        trie.insert(*key);
        trie.pretty_print();
    }
    
    println!("\nfinal structure:");
    trie.pretty_print();
    
    println!("testing predecessor queries:");
    let queries = vec![8, 13, 20];
    for query in queries {
        if let Some(pred) = trie.predecessor(query) {
            if let Ok(pred_guard) = pred.read() {
                println!("predecessor of {} is {}", query, pred_guard.key);
            }
        } else {
            println!("predecessor of {} is None", query);
        }
    }
}