use std::collections::HashMap;
//use std::cmp::Ordering;

// use rust_utils::containers::binary_heap::BinaryHeap;

#[derive(Debug)]
pub struct ChangeablePriorityQueue<T> {
    nodes: Vec<T> ,
    hash: HashMap<T,usize>,
}
impl<T:Ord + Eq + std::hash::Hash + Clone> ChangeablePriorityQueue<T> {
    pub fn new() -> Self {
        ChangeablePriorityQueue {
            nodes: Vec::new(),
            hash: HashMap::new(),
        }
    }
    pub fn push(&mut self, node: T) {
        self.nodes.push(node) ;
        self.heapify_up(self.nodes.len()-1) ;
    }
    pub fn heapify_up(&mut self, mut index: usize) {
        while index>0 {
            let parent = (index-1)/2 ;
            if self.nodes[index]> self.nodes[parent] {
                self.hash.insert(self.nodes[parent].clone(),index) ;
                self.hash.insert(self.nodes[index].clone(),parent) ;
                self.nodes.swap(index,parent) ;
                index = parent ;
            }
            else 
            {
                break;
            }
        }
    }
    pub fn pop(&mut self)-> Option<T> {
        //removing the root element from the tree where Option means the value can be some(T) or NONE
        let len = self.nodes.len() ;
       if len ==0 {
           None 
       }
       else if len == 1 {
        let value = self.nodes.pop().unwrap() ;
        self.hash.remove(&value) ;
        Some(value)     
       }
       else {
           self.hash.insert(self.nodes[0].clone(),len-1) ;
           self.hash.insert(self.nodes[len-1].clone(),0) ;
           
           self.nodes.swap(0,len-1) ;
           let min = self.nodes.pop() ;
           self.heapify_down(0) ;
           min 
       }
   }
   pub fn heapify_down(&mut self, index: usize) {
    loop {
        let  left_child = 2*index + 1 ;
        let  right_child = 2*index + 2 ;

        let mut smallest = index ; //as we are implementing min heap

        if left_child<self.nodes.len() && self.nodes[left_child] < self.nodes[smallest] {
            smallest = left_child ;
        }
        if right_child<self.nodes.len() && self.nodes[right_child] < self.nodes[smallest] {
            smallest = right_child ;
        }

        if smallest != index {
            self.hash.insert(self.nodes[smallest].clone(),index) ;
            self.hash.insert(self.nodes[index].clone(),smallest) ;
            self.nodes.swap(smallest,index) ;
        }
        else {
            break ;
        }
    }
    }
//You can use both remove and push function to update the priority of any node
    pub fn remove(&mut self, node:T) {
        if self.nodes.len() == 0 {
            return 
        }
        let len = self.nodes.len()-1 ;
        if let Some(&position) = self.hash.get(&node) {
            self.hash.remove(&node) ;
            if len == 0 {
                self.nodes.pop() ;
                return 
            }
            self.nodes.swap(position,len) ;
            self.nodes.pop() ;
            self.heapify_down(position) ;
        }
        
    }
   
}
