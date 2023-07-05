use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Vertex {
    id: usize,
    distance: i32,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Graph {
    vertices: usize,
    adjacency_list: HashMap<usize, Vec<(usize, i32)>>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, weight: i32) {
        self.adjacency_list
            .entry(u)
            .or_insert(Vec::new())
            .push((v, weight));
        self.adjacency_list
            .entry(v)
            .or_insert(Vec::new())
            .push((u, weight));
    }

    fn dijkstra(&self, source: usize, destination: usize) -> Option<Vec<usize>> {
        let mut distances: Vec<_> = (0..self.vertices).map(|_| i32::MAX).collect();
        let mut previous: Vec<_> = (0..self.vertices).map(|_| None).collect();
        let mut priority_queue = BinaryHeap::new();

        distances[source] = 0;
        priority_queue.push(Vertex {
            id: source,
            distance: 0,
        });

        while let Some(Vertex { id, distance }) = priority_queue.pop() {
            if id == destination {
                let mut path = vec![id];
                let mut current = id;
                while let Some(prev) = previous[current] {
                    path.push(prev);
                    current = prev;
                }
                path.reverse();
                return Some(path);
            }

            if distance > distances[id] {
                continue;
            }

            if let Some(neighbors) = self.adjacency_list.get(&id) {
                for (neighbor, weight) in neighbors {
                    let new_distance = distance + weight;
                    if new_distance < distances[*neighbor] {
                        distances[*neighbor] = new_distance;
                        previous[*neighbor] = Some(id);
                        priority_queue.push(Vertex {
                            id: *neighbor,
                            distance: new_distance,
                        });
                    }
                }
            }
        }

        None
    }
}

fn main() {
    let mut graph = Graph::new(6);
    graph.add_edge(0, 1, 7);
    graph.add_edge(0, 2, 9);
    graph.add_edge(0, 5, 14);
    graph.add_edge(1, 2, 10);
    graph.add_edge(1, 3, 15);
    graph.add_edge(2, 3, 11);
    graph.add_edge(2, 5, 2);
    graph.add_edge(3, 4, 6);
    graph.add_edge(4, 5, 9);

    let source = 0;
    let destination = 4;

    if let Some(path) = graph.dijkstra(source, destination) {
        println!("Shortest path from {} to {}:", source, destination);
        for vertex in path {
            print!("{} ", vertex);
        }
    } else {
        println!("No path found from {} to {}.", source, destination);
    }
}
