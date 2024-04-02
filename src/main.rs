use std::{collections::VecDeque, vec};


const DOMAIN_MAX: usize = 3;
const DOMAIN_EMPTY: u8 = 0;

#[derive(PartialEq)]
struct Node {
    value: i32,
    domain: [u8; DOMAIN_MAX],
}

struct Edge<'a> {
    first: &'a Node,
    second: &'a Node
}

fn get_neighbours(node: &Node) -> Vec<Node> {
    vec![]
}

fn is_domain_valid(edge: &Edge, first_domain: u8, second_domain: u8) -> bool {
    true
}

fn review(psr: &Vec<Node>, mut edge: &Edge) -> bool {
    let mut reviewed = false;

    for i in 0..DOMAIN_MAX {
        let first_domain = edge.first.domain[i]; 
        if first_domain == DOMAIN_EMPTY {
            continue;
        }
        let mut valid = false;
        for second_domain in edge.second.domain.iter().filter(|d| **d != DOMAIN_EMPTY) {
            if is_domain_valid(edge, first_domain, *second_domain) {
                valid = true;
                break;
            }
        }

        if !valid {
            // edge.first.domain[i] = DOMAIN_EMPTY;
            reviewed = true;
        }
    }

    false
}

fn ac3(psr: &Vec<Node>) -> bool {
    let mut queue: VecDeque<Edge> = VecDeque::new();
    // TODO add all edges in queue
    while !queue.is_empty() {
        let mut edge = queue.pop_front().unwrap();

        if review(psr,&edge) {
            if edge.first.domain.iter().all(|d| *d == DOMAIN_EMPTY) {
                return false;
            }
            for neighbour in get_neighbours(edge.first) {
                if &neighbour == edge.second {
                    continue;
                }
                let new_edge = Edge {
                    first: edge.first, // TODO: add neighbour ref,
                    second: edge.first
                };
                queue.push_back(new_edge);
            }
        }

    }

    true
}


fn main() {
    println!("Hello, world!");
}
