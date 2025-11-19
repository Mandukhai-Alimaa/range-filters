use crate::Key;

pub fn longest_common_prefix_length(key1: Key, key2: Key) -> u32 {
    (key1 ^ key2).leading_zeros()
}