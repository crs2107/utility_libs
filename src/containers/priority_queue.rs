//<T> means a generic data type parameter i.e you can use PriorityQueue with any data types
#[derive(Default,Debug)]
pub struct PriorityQueue<T> {
    nodes: Vec<T>,
}

impl<T: Ord> PriorityQueue<T> { //<T: Ord> means that the data type must possess the trait Ord i.e there should exist an ordering
    pub fn new() -> Self {
        PriorityQueue{nodes: Vec::new()}
    }//constructor 

    pub fn push(&mut self, value: T) {
        self.nodes.push(value) ;
        self.heapify_up( self.nodes.len()-1) ; //adding the value at the last index and calling heapify up
    }

    pub fn pop(&mut self)-> Option<T> {
         //removing the root element from the tree where Option means the value can be some(T) or NONE
         let len = self.nodes.len() ;
        if len ==0 {
            None 
        }
        else if len == 1 {
            self.nodes.pop() 
        }
        else {
            self.nodes.swap(0,len-1) ;
            let min = self.nodes.pop() ;
            self.heapify_down(0) ;
            min 
        }
    }
    pub fn heapify_up(&mut self, mut index: usize) {
        while index>0 {
            let parent = (index-1)/2 ;
            if self.nodes[index]> self.nodes[parent] {
                self.nodes.swap(index,parent) ;
                index = parent ;
            }
            else 
            {
                break;
            }
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
                self.nodes.swap(smallest,index) ;
            }
            else {
                break ;
            }
        }
    }

    #[allow(dead_code)]
    pub fn decrease_key(&mut self,  index: usize, new_key: T) { //decreasing the priority
        if index >= self.nodes.len() {//index is out of bound
            return
        }
        if new_key >= self.nodes[index] { //new_key is bigger
            return
        }

        self.nodes[index] = new_key ;
        self.heapify_up(index) ;

    }

}
