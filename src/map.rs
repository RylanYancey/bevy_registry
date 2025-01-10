use std::cmp::Ordering;

pub(super) struct Node {
    key: u64,
    val: u16,
    gt: Option<u16>,
    lt: Option<u16>,
}

/// An Insert-only densely packed map. 
/// I'm using this to minimize allocations
/// during program startup. BTreeMap heap allocates
/// for every element. We don't need fast removal
/// so we don't care. 
pub struct InsertMap {
    pub(super) entries: Vec<Node>,
}

impl InsertMap {
    pub fn insert(&mut self, key: u64, val: u16) -> Option<u16> {
        let index = self.entries.len();
        if index == 0 {
            self.entries.push(Node {
                key, val, gt: None, lt: None
            });
            return None;
        }

        let mut i = 0;
        loop {
            let next = match key.cmp(&self.entries[i].key) {
                Ordering::Greater => &mut self.entries[i].gt,
                Ordering::Less => &mut self.entries[i].lt,
                Ordering::Equal => return Some(i as u16),
            };

            if next.is_none() {
                *next = Some(index as u16);
                self.entries.push(Node { key, val, gt: None, lt: None });
                return None;
            } else {
                i = next.unwrap() as usize;
            }
        }
    }

    pub fn get(&self, key: u64) -> Option<u16> {
        let mut i = 0;
        loop {
            i = match key.cmp(&self.entries[i].key) {
                Ordering::Greater => self.entries[i].gt?.into(),
                Ordering::Less => self.entries[i].lt?.into(),
                Ordering::Equal => return Some(self.entries[i].val),
            };
        }
    }
}