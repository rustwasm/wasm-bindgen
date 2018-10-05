use std::mem;

type T = usize;

const BITS: usize = mem::size_of::<T>() * 8;

pub struct BitSet {
    bits: Vec<T>,
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet { bits: Vec::new() }
    }

    pub fn insert(&mut self, i: u32) -> bool {
        let i = i as usize;
        let idx = i / BITS;
        let bit = 1 << (i % BITS);
        if self.bits.len() <= idx {
            self.bits.resize(idx + 1, 0);
        }
        let slot = &mut self.bits[idx];
        if *slot & bit != 0 {
            false
        } else {
            *slot |= bit;
            true
        }
    }

    pub fn contains(&self, i: &u32) -> bool {
        let i = *i as usize;
        let idx = i / BITS;
        let bit = 1 << (i % BITS);
        self.bits.get(idx)
            .map(|x| *x & bit != 0)
            .unwrap_or(false)
    }
}

impl Default for BitSet {
    fn default() -> BitSet {
        BitSet::new()
    }
}

#[cfg(test)]
mod tests {
    use super::BitSet;

    #[test]
    fn simple() {
        let mut x = BitSet::new();
        assert!(!x.contains(&1));
        assert!(!x.contains(&0));
        assert!(!x.contains(&3));
        assert!(x.insert(3));
        assert!(x.contains(&3));
        assert!(!x.insert(3));
        assert!(x.contains(&3));
        assert!(!x.contains(&1));
        assert!(x.insert(2));
        assert!(x.contains(&2));
    }
}
