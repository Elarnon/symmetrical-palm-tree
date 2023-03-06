use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // NB: we want a min-heap, not a max-heap
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

fn main() {
    let mut lines = io::stdin().lines().map(|l| l.unwrap());
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let shortcuts = lines.next().unwrap();
    let shortcuts = shortcuts
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    let mut dist = (0..n).map(|_| usize::MAX).collect::<Vec<_>>();
    dist[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: 0,
    });

    let mut neighbors = Vec::with_capacity(3);
    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        neighbors.clear();
        if position + 1 < n {
            neighbors.push(position + 1);
        }

        if position > 0 {
            neighbors.push(position - 1);
        }

        neighbors.push(shortcuts[position]);

        for &next_position in neighbors.iter() {
            if cost + 1 < dist[next_position] {
                heap.push(State {
                    cost: cost + 1,
                    position: next_position,
                });
                dist[next_position] = cost + 1;
            }
        }
    }

    for n in dist.iter() {
        print!("{} ", n);
    }
    println!("");
}
