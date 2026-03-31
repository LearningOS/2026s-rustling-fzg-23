/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default+PartialOrd,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
    min:bool,
}

impl<T> Heap<T>
where
    T: Default+PartialOrd,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
            min:false,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.right_child_idx(idx) >= self.count {
            return self.left_child_idx(idx);
        }
        if self.items[self.right_child_idx(idx)] > self.items[self.left_child_idx(idx)] {
            return self.left_child_idx(idx);
        }
        else{
            return self.right_child_idx(idx);
        }
    }

    fn max_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.right_child_idx(idx) >= self.count {
            return self.left_child_idx(idx);
        }
        if self.items[self.right_child_idx(idx)] > self.items[self.left_child_idx(idx)] {
            return self.right_child_idx(idx);
        }
        else{
            return self.left_child_idx(idx);
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
     Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
       Self::new(|a, b| a < b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        } else {
            if self.min==false{
                for i in 0..self.len()-1 {
                    for j in 1..(self.len()-i).max(2) {
                        if self.items[j] < self.items[j + 1] {
                            self.items.swap(j, j + 1)
                        }
                    }
                }
            }
            else if self.min==true{
                for i in 1..(self.len()-1).max(2) {
                    if self.items[i] > self.items[i+1] {
                        self.items.swap(i, i+1)
                    }
                }
            }
            let re=self.items.remove(1);
            self.count -= 1;
            Some(re)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        let mut x= Heap::new(|a, b| a < b);
        x.min=true;
        x
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        let mut x= Heap::new(|a, b| a > b);
        x.min=false;
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}