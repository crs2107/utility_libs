use std::collections::{HashMap};
use std::cmp::Ordering;

use binary_heap::BinaryHeap ;
mod binary_heap ;





#[derive(Debug,Eq,PartialEq)]
struct Element<T> {
    key : T,
    priority : usize
}
impl<T: Ord> Ord for Element<T>{
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Reverse for max-heap
    }
}

impl<T: Ord> PartialOrd for Element<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct ChangeablePriorityQueue<T> {
    heap: BinaryHeap<Element<T>> ,
    pub map: HashMap<T, usize> ,
}
impl<T:Ord + Eq + std::hash::Hash + Clone> ChangeablePriorityQueue<T> {
    pub fn new() -> Self {
        ChangeablePriorityQueue {
            heap: BinaryHeap::new(),
            map: HashMap::new(),
        }

    }
    pub fn push(&mut self, key: T, priority: usize) {
        let element = Element { key: key.clone(), priority };
        self.map.insert(key, priority);
        self.heap.push(element);
    }
    pub fn change_priority(&mut self, key: T, new_priority: usize) {
        if let Some(&current_priority) = self.map.get(&key) {
            self.map.remove(&key) ;
            self.push(key,new_priority) ;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        while let Some(element) = self.heap.pop() {
            if let Some(&priority) = self.map.get(&element.key) {
                if priority == element.priority {
                    self.map.remove(&element.key);
                    return Some(element.key);
                }
            }
        }
        None
    }

    }
