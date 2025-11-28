use range_filters::data_gen::generate_smooth_u64;
use range_filters::y_fast_trie::YFastTrie;
use range_filters::U64_BITS;

fn main() {
    let mut keys = generate_smooth_u64(Some(1000));
    keys.sort();
    println!("keys: {:?}", keys);
    
    let y_fast_trie = YFastTrie::new_with_keys(&keys, U64_BITS);
    
    println!("Keys 99, 100, 101: {:?}", &keys[99..102]);
    println!("key {} contains: {}", keys[100], y_fast_trie.contains(keys[100]));
    println!("key {} predecessor: {:?}", keys[100], y_fast_trie.predecessor(keys[100] - 1));
    println!("key {} successor: {:?}", keys[100], y_fast_trie.successor(keys[100] + 1));
    // println!("y-fast trie: {:?}", y_fast_trie);

}