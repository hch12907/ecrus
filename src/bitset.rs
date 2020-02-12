pub struct BitSet {
    set: Vec<usize>,
}

impl BitSet {
    const fn ptr_size() -> usize { std::mem::size_of::<usize>() * 8 }

    pub fn new() -> Self {
        Self { set: Vec::new() }
    }

    pub fn with_capacity(size: usize) -> Self {
        let size = (size + Self::ptr_size() - 1) / Self::ptr_size();
        Self { set: Vec::with_capacity(size) }
    }

    pub fn with_capacity_zeroed(size: usize) -> Self {
        let size = (size + Self::ptr_size() - 1) / Self::ptr_size();
        Self { set: vec![0; size] }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let vec_idx = index / Self::ptr_size();
        let int_idx = index % Self::ptr_size();
        debug_assert!(vec_idx < self.set.len(), "index larger than capacity!");
        
        if vec_idx < self.set.len() {
            self.set[vec_idx] |= (value as usize) << int_idx;
        }
    }

    pub fn get(&self, index: usize) -> bool {
        let vec_idx = index / Self::ptr_size();
        let int_idx = index % Self::ptr_size();
        debug_assert!(vec_idx < self.set.len(), "index larger than capacity!");

        if vec_idx < self.set.len() {
            ((self.set[vec_idx] >> int_idx) & 1) == 1
        } else {
            false
        }
    }

    pub fn reserve(&mut self, size: usize) {
        let size = (size + Self::ptr_size() - 1) / Self::ptr_size();
        self.set.reserve(size);
    }
}
