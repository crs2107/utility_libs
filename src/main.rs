use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;
//use binary_heap::BinaryHeap ;
use changeable_priority_queue::ChangeablePriorityQueue ;

mod changeable_priority_queue;

fn main() {
    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Open the input file
    let file = File::open(file_path).expect("Failed to open file");

    // Create a buffer reader to read lines from the file
    let reader = io::BufReader::new(file);

    // Read the number of vertices and edges from the first line of the file
    let mut lines_iter = reader.lines();
    let first_line = lines_iter.next().expect("Failed to read first line").unwrap();
    let mut parts = first_line.split_whitespace();
    let num_vertices: usize = parts.next().unwrap().parse().expect("Please enter a number");
    let num_edges: usize = parts.next().unwrap().parse().expect("Please enter a number");

    // Create a graph representation
    let mut graph = vec![vec![0; num_vertices]; num_vertices];
    

    // Read the edges and their weights from the file
    for line in lines_iter.take(num_edges) {
        let line = line.expect("Failed to read line");
        let mut parts = line.split_whitespace();
        let source: usize = parts.next().unwrap().parse().expect("Please enter a number");
        let destination: usize = parts.next().unwrap().parse().expect("Please enter a number");
        let weight: usize = parts.next().unwrap().parse().expect("Please enter a number");
        graph[source][destination] = weight;
    }

    // Example Dijkstra's algorithm implementation
    // Replace this with your own implementation

    // Print the graph
    println!("Graph:");
    for row in &graph {
        println!("{:?}", row);
    }
    let mut input = String::new() ;
    println!("input the source vertex:");
    io::stdin().read_line(&mut input).expect("failed to read input");

    let source: usize = input.trim().parse().expect("invalid input");

    if source >= num_vertices {
        println!("Source vertex does not exist in the graph") ;
        return 
    }


    let sssp = dijkstra_implement(&graph,num_vertices, source);
    println!("the shortest distances from {} are {:?}",source, sssp); 


    let sssp1 = dijkstra_using_priority_queue(&graph,num_vertices, source);
    println!("the shortest distances from {} are {:?}",source, sssp1); 


    
}


#[derive(Debug, PartialEq, Eq)]
struct VerticesWithSD {
    vertex : usize,
    shortest_distance : usize,
}
impl PartialOrd for VerticesWithSD {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for VerticesWithSD {
    fn cmp(&self, other: &Self) -> Ordering {
        self.shortest_distance.cmp(&other.shortest_distance)
    }
}

fn dijkstra_using_priority_queue(graph: &Vec<Vec<usize>>, number_of_vertices: usize, source: usize)-> Vec<usize> {

    let mut dist: Vec<usize>= vec![usize::MAX; number_of_vertices] ;
    //let mut pq = BinaryHeap::<VerticesWithSD>::new();
    let mut pq = ChangeablePriorityQueue::new() ; 

    dist[source] = 0 ;

    pq.push(source,0); 
    while let Some(vertex)=pq.pop() {
        let current_distance = dist[vertex] ;

        for v in 0..number_of_vertices {
            if graph[vertex][v] != 0 {
                let new_distance = current_distance + graph[vertex][v] ;
                if dist[v] > new_distance {
                    dist[v]=new_distance ;
                    //pq.push(VerticesWithSD{vertex: v, shortest_distance: new_distance}) ;
                    pq.change_priority(vertex, new_distance) ;

                }

            }

            
        }
    }
   
    dist

}

//find the vertex with minimum distance  from the set of vertices that are not in the shortest path tree
fn min_distance(dist : &Vec<usize> , sptset: &Vec<bool>, number_of_vertices: usize)-> usize {
    let mut min = usize::MAX ;
    let mut min_index = 0 ;

    for v in 0..number_of_vertices {
        if sptset[v] == false && dist[v] <= min {
            min = dist[v] ;
            min_index = v ;
        }
    }
    min_index
}


fn dijkstra_implement(graph: &Vec<Vec<usize>>, number_of_vertices: usize, source: usize) -> Vec<usize> {
    let mut distance: Vec<usize>= vec![usize::MAX; number_of_vertices] ; //the output array with d(source,vertex) = infinity
    //for all vertex

    let mut spt_set: Vec<bool> = vec![false; number_of_vertices] ;//the shortest path tree array that indicates if the vertex 
    //is included in the shortest path tree or not

    distance[source]=0 ;//d(source,source)=0

    for _count in 0..number_of_vertices-1 {
        let u = min_distance(&distance, &spt_set, number_of_vertices); //taking the min distance vertex
        spt_set[u] = true ; //including it in the shortest path tree

        for v in 0..number_of_vertices {
            //triangle inequality
            if spt_set[v]== false && graph[u][v]!= 0 && distance[u] != usize::MAX && distance[u] + graph[u][v] < distance[v] {
                distance[v] = distance[u] + graph[u][v] ;
            }
        }

    }

     return distance ;
}
