struct Heap {
    harr: Vec<i32>,
    size: usize,
    capacity: usize
}

impl Heap {
    pub fn new() -> Self<> {
        return Heap {
            harr: vec![0;5],
            size: 0,
            capacity: 5,
        }
    }

    pub fn push(&mut self, v: i32) {
        if self.capacity == self.len() {
            self.harr.resize(self.capacity * 2, 0);
            self.capacity = self.capacity * 2;
        }
        let mut i = self.len();
        self.harr[i] = v;
        self.size += 1;

        while i != 0 && self.harr[self.parent(i)] > self.harr[i] {
            let parent = self.parent(i);
            self.harr.swap(parent, i);
            i = parent;
        }
    }

    pub fn pop(&mut self) -> Option<i32>{
        if self.len() == 0 {
            return None;
        }
        if self.len() == 1 {
            self.size -= 1;
            let result = self.harr[0];
            self.harr[0] = 0;
            return Some(result);
        }
        let result = self.harr[0];
        let swap_target = self.len() - 1;
        self.harr.swap(0, swap_target);
        self.harr[swap_target] = 0;
        self.size -= 1;

        let mut i = 0;
        loop {
            let left = self.left(i);
            let right = self.right(i);

            if left >= self.len() || right >= self.len() {
                break;
            }

            let mut min_index = i;
            if self.harr[min_index] > self.harr[left] {
                min_index = left;
            }
            if self.harr[min_index] > self.harr[right] {
                min_index = right;
            }

            if min_index != i {
                self.harr.swap(min_index, i);
                i = min_index;
            } else {
                break;
            }
        }

        Some(result)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    fn parent(&self, i: usize) -> usize {
        (i-1)/2
    }

    fn left(&self, i: usize) -> usize {
        2*i+1
    }

    fn right(&self, i: usize) -> usize {
        2*i+2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_heap() {
        let h = Heap::new();
        assert_eq!(0, h.len());
    }

    #[test]
    fn pop() {
        let mut h = Heap::new();
        h.push(1);
        h.push(2);
        h.push(3);
        h.push(4);

        assert_eq!(1, h.pop().unwrap());
        assert_eq!(2, h.pop().unwrap());
        assert_eq!(3, h.pop().unwrap());
        assert_eq!(4, h.pop().unwrap());
        assert!(h.pop().is_none());

    }

    #[test]
    fn push() {
       let mut h = Heap::new() ;
        h.push(5);

        assert_eq!(1, h.len());
        assert_eq!(5, h.harr[0]);

        h.push(2);
        assert_eq!(2, h.harr[0]);
        assert_eq!(5, h.harr[1]);

        h.push(3);
        assert_eq!(3, h.harr[2]);

        h.push(1);
        assert_eq!(1, h.harr[0]);
        assert_eq!(2, h.harr[1]);
        assert_eq!(5, h.harr[h.len() - 1]);

        h.push(4);
        h.push(6);
        assert_eq!(6, h.len());
    }
}
