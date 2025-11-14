use std::sync::{Arc, RwLock, Weak};
use dashmap::DashMap;

pub type Key = u64;

// placeholder
#[derive(Debug, Default)]
pub struct BSTGroup;

#[derive(Debug)]
pub struct XFastTrie {
    pub levels: Vec<XFastLevel>,
    // representatives
    // pub reps: HashMap<Key, Arc<RwLock<RepNode>>>,
    pub head_rep: Option<Arc<RwLock<RepNode>>>,
    pub tail_rep: Option<Arc<RwLock<RepNode>>>,
    
    // no. of levels = no. of bits in the keys
    pub no_levels: u8,
}

#[derive(Debug, Default, Clone)]
pub struct XFastLevel {
    pub table: DashMap<u64, XFastValue>
}

#[derive(Debug, Default, Clone)]
pub struct XFastValue {

    pub left_child: Option<Arc<RwLock<XFastValue>>>,
    pub right_child: Option<Arc<RwLock<XFastValue>>>,

    pub representative: Option<Arc<RwLock<RepNode>>>
}

#[derive(Debug, Default, Clone)]
pub struct RepNode {
    pub key: Key,
    pub left: Option<Weak<RwLock<RepNode>>>,
    pub right: Option<Weak<RwLock<RepNode>>>,
    pub bucket: Option<Arc<RwLock<BSTGroup>>>,
}

impl XFastTrie {
    pub fn new(no_levels: u8) -> Self {
        Self {
            levels: vec![XFastLevel::default(); no_levels as usize + 1],
            head_rep: None,
            tail_rep: None,
            no_levels,
        }
    }

    // fn contains(&self, key: Key) -> bool {
    //     self.reps.contains_key(&key)
    // }

    fn predecessor(&self, key: Key) -> Option<Arc<RwLock<RepNode>>> {
        let mut low = 0;
        let mut high = self.no_levels;

        while low < high {
            let mid = (low + high + 1) / 2;
            let prefix = key >> (self.no_levels - mid);
            if self.levels[mid as usize].table.contains_key(&prefix) {
                low = mid;
            }
            else {
                high = mid - 1;
            }
        }

        let best_level = low;

        if best_level == 0 {
            return self.head_rep.clone();
        }

        let prefix = key >> (self.no_levels - best_level);


        let x_fast_value = self.levels[best_level as usize].table.get(&prefix)?;

        if let Some(representative) = &x_fast_value.representative {
            if let Ok(rep) = representative.read() {
                if rep.key <= key {
                    return Some(representative.clone());
                } else {
                    // need to find predecessor by traversing left
                    if let Some(left_weak) = &rep.left {
                        return left_weak.upgrade();
                    }
                }
            }
        }

        None
    }

    fn successor(&self, key: Key) -> Option<Arc<RwLock<RepNode>>> {
        let mut low = 0;
        let mut high = self.no_levels;

        while low < high {
            let mid = (low + high + 1) / 2;
            let prefix = key >> (self.no_levels - mid);
            if self.levels[mid as usize].table.contains_key(&prefix) {
                low = mid;
            }
            else {
                high = mid - 1;
            }
        }

        let best_level = low;

        if best_level == 0 {
            return self.tail_rep.clone();
        }

        let prefix = key >> (self.no_levels - best_level);


        let x_fast_value = self.levels[best_level as usize].table.get(&prefix)?;

        if let Some(representative) = &x_fast_value.representative {
            if let Ok(rep) = representative.read() {
                if rep.key >= key {
                    return Some(representative.clone());
                } else {
                    // need to find successor by traversing right
                    if let Some(right_weak) = &rep.right {
                        return right_weak.upgrade();
                    }
                }
            }
        }

        None
    }


    // insert a key into the x-fast trie
    fn insert(&mut self, key: Key) {

        // step 1: find the longest prefix
        let mut low = 0;
        let mut high = self.no_levels;

        while low < high {
            let mid = (low + high + 1) / 2;
            let prefix = key >> (self.no_levels - mid);
            if self.levels[mid as usize].table.contains_key(&prefix) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }

        let best_level = low;

        // step 2: create representative
        let representative = Arc::new(RwLock::new(RepNode {
            key,
            left: None,
            right: None,
            bucket: None,
        }));

        // step 3: create child prefixes from best_level+1 to no_levels
        for level in (best_level + 1)..=self.no_levels {
            let prefix = key >> (self.no_levels - level);
            let new_x_fast_value = XFastValue {
                left_child: None,
                right_child: None,
                representative: Some(representative.clone()),
            };
            self.levels[level as usize].table.insert(prefix, new_x_fast_value.clone());

            // update parent's child pointers
            if level > 1 {
                let parent_prefix = key >> (self.no_levels - (level - 1));
                if let Some(mut parent_value) = self.levels[(level - 1) as usize].table.get_mut(&parent_prefix) {
                    let bit = (key >> (self.no_levels - level)) & 1;
                    if bit == 0 {
            
                        parent_value.left_child = Some(Arc::new(RwLock::new(new_x_fast_value.clone())));
                    } else {
                        parent_value.right_child = Some(Arc::new(RwLock::new(new_x_fast_value.clone())));
                    }
                }
            }
        }

        // step 4: update linked list pointers
        let predecessor = self.predecessor(key);
        let successor = self.successor(key);
        
        // update predecessor's right pointer
        if let Some(pred) = &predecessor {
            if let Ok(mut pred_guard) = pred.write() {
                pred_guard.right = Some(Arc::downgrade(&representative));
            }
        }
        
        // update successor's left pointer  
        if let Some(succ) = &successor {
            if let Ok(mut succ_guard) = succ.write() {
                succ_guard.left = Some(Arc::downgrade(&representative));
            }
        }
        
        // set representative's pointers
        if let Ok(mut rep_guard) = representative.write() {
            rep_guard.left = predecessor.as_ref().map(|p| Arc::downgrade(p));
            rep_guard.right = successor.map(|s| Arc::downgrade(&s));
            rep_guard.bucket = Some(Arc::new(RwLock::new(BSTGroup::default())));
        }

        // step 5: update head and tail representatives
        let should_update_head = if let Some(head_rep) = &self.head_rep {
            if let Ok(head) = head_rep.read() {
                head.key > key
            } else {
                false
            }
        } else {
            // first key being inserted
            true 
        };
        
        if should_update_head {
            self.head_rep = Some(representative.clone());
        }
        
        let should_update_tail = if let Some(tail_rep) = &self.tail_rep {
            if let Ok(tail) = tail_rep.read() {
                tail.key < key
            } else {
                false
            }
        } else {
            // first key being inserted
            true 
        };
        
        if should_update_tail {
            self.tail_rep = Some(representative.clone());
        }
    }
}