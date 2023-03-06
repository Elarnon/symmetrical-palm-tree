use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

// A State represents a potential path from node 0 to the node position with cost cost, where the
// cost represents the energy.
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // NB: we want a min-heap, not a max-heap, so we need to flip the `cost` order.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// This is a helper struct that allows to compute Dijkstra's shortest-path on a graph over a
// bi-directional line.
//
// Conceptually, the graph is pre-populated with the elements 0 .. n with bi-directional edges from
// i to i + 1 and i - 1.
//
// The starting point of the graph is always node 0.
//
// Additional edges can be added by the user and are handled through a combination of the pop and
// push methods.
//
// Calling pop() advances the search, and returns the a potential path to examine. The user should
// apply push any possible transition to the corresponding node, which will be added to the solver
// if they improve its current cost.
//
// Once pop() returns None, the solver has examined all potential paths from the start position to
// any other position:
struct LinearDijkstra {
    // The dist vector maps each 0-indexed node to the current shortest distance to that node. We
    // know that there is at least a path of length i to node i by walking in a straight line.
    distances: Vec<usize>,

    // The heap is used to implement a priority queue, so that we always investigate short paths
    // before long paths (that could end up being discarded).
    heap: BinaryHeap<State>,
}

impl LinearDijkstra {
    // Create a new LinearDijkstra solver with n nodes representings the integers 0 to n - 1.
    fn new(n: usize) -> Self {
        let mut heap = BinaryHeap::new();
        let mut distances = (0..n).map(|_| usize::MAX).collect::<Vec<_>>();

        // We start on node 0 with 0 cost.
        distances[0] = 0;
        heap.push(State {
            cost: 0,
            position: 0,
        });

        LinearDijkstra { distances, heap }
    }

    // Discover a new potential path.
    //
    // The new potential path is only considered if it has lowest total cost than any current path
    // to the node.
    fn push(&mut self, state: State) {
        if state.cost < self.distances[state.position] {
            self.distances[state.position] = state.cost;
            self.heap.push(state);
        }
    }

    // Examine a potential path that could lead to an improvement.
    //
    // If appropriate, the neighbours (position - 1 and position + 1) of the new potential path
    // will automatically be added to the solver. The user should then add any additional shortcuts
    // that are available before calling pop again.
    fn pop(&mut self) -> Option<State> {
        while let Some(State { cost, position }) = self.heap.pop() {
            // If we have already found a shorter path to that node, we can safely skip this one.
            if cost > self.distances[position] {
                continue;
            }

            // We can move forward if we are not at the end
            if position + 1 < self.distances.len() {
                self.push(State {
                    cost: cost + 1,
                    position: position + 1,
                });
            }

            // We can move backward if we are not at the start
            if position > 0 {
                self.push(State {
                    cost: cost + 1,
                    position: position - 1,
                });
            }

            return Some(State { cost, position });
        }

        None
    }
}

fn main() {
    // Parsing. Note that we subtract 1 from the shortcut index because that works better with
    // 0-indexed arrays.
    let mut lines = io::stdin().lines().map(|l| l.unwrap());
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let shortcuts = lines.next().unwrap();
    let shortcuts = shortcuts
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    // Solving.
    //
    // We use a LinearDijkstra solver and add in the shortcut paths.
    let mut solver = LinearDijkstra::new(n);
    while let Some(State { cost, position }) = solver.pop() {
        solver.push(State {
            cost: cost + 1,
            position: shortcuts[position],
        });
    }

    // Once pop() returns None, we have examined all possible paths: we just have to print the
    // output.
    //
    // Note that going to the first position always has a cost of 0, which we use to intersperse
    // the spaces.
    print!("0");
    for n in solver.distances.iter().skip(1) {
        print!(" {}", n);
    }
    println!("");
}
