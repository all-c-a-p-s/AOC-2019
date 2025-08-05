use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Planet {
    name: String,
    orbits: Box<Option<Planet>>,
    orbit_count: usize,
}

impl Planet {
    fn new(name: String) -> Planet {
        Self {
            name: name,
            orbits: Box::new(None),
            orbit_count: 0,
        }
    }
}

fn read_input() -> Vec<(String, String)> {
    include_str!("day06.in")
        .lines()
        .map(|x| x.split_once(")").unwrap())
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}

// returns leaf nodes of trees
fn build_trees(v: &Vec<(String, String)>) -> Vec<Planet> {
    let mut leaf_nodes: Vec<(Planet, bool)> = {
        let mut blocked = HashSet::new();
        let mut candidates = HashSet::new();
        for (a, b) in v {
            blocked.insert(b);
            if candidates.contains(b) {
                candidates.remove(b);
            }

            if !blocked.contains(a) {
                candidates.insert(a);
            }
        }
        candidates
            .into_iter()
            .map(|name| (Planet::new(name.clone()), false))
            .collect()
    };

    let mut final_leafs = vec![];

    while !leaf_nodes.is_empty() {
        let mut new_leaf_nodes = vec![];
        for (a, b) in v {
            for (n, y) in leaf_nodes.iter_mut() {
                if *a == n.name {
                    let mut np = Planet::new(b.clone());
                    np.orbit_count = n.orbit_count + 1;
                    np.orbits = Box::new(Some(n.clone()));

                    new_leaf_nodes.push((np, false));
                    *y = true;
                }
            }
        }

        for (x, has_child) in leaf_nodes {
            if !has_child {
                final_leafs.push(x);
            }
        }

        leaf_nodes = new_leaf_nodes;
    }

    final_leafs
}

// returns leaf nodes of trees
fn build_trees_2(v: &Vec<(String, String)>) -> Vec<Planet> {
    let mut leaf_nodes: Vec<Planet> = {
        let mut blocked = HashSet::new();
        let mut candidates = HashSet::new();
        for (a, b) in v {
            blocked.insert(b);
            if candidates.contains(b) {
                candidates.remove(b);
            }

            if !blocked.contains(a) {
                candidates.insert(a);
            }
        }
        candidates
            .into_iter()
            .map(|name| Planet::new(name.clone()))
            .collect()
    };

    let mut end_points = vec![];

    while !leaf_nodes.is_empty() {
        let mut new_leaf_nodes = vec![];
        for (a, b) in v {
            for n in leaf_nodes.iter_mut() {
                if *a == n.name {
                    let mut np = Planet::new(b.clone());
                    np.orbit_count = n.orbit_count + 1;
                    np.orbits = Box::new(Some(n.clone()));

                    new_leaf_nodes.push(np.clone());

                    if *b == "YOU".to_string() || *b == "SAN".to_string() {
                        end_points.push(np);
                    }
                }
            }
        }

        leaf_nodes = new_leaf_nodes;
    }

    end_points
}

pub fn part_one() -> i32 {
    let parsed = read_input();
    let mut leaf_nodes = build_trees(&parsed);

    let mut seen = HashSet::new(); // avoid double counting nodes with 2 or more children

    let mut total = 0;
    while !leaf_nodes.is_empty() {
        let mut new_leaf_nodes = vec![];
        for n in &leaf_nodes {
            total += n.orbit_count;
            let binding = *n.clone().orbits;
            if let Some(n) = binding {
                if !seen.contains(&n.name) {
                    new_leaf_nodes.push(n.clone());
                    seen.insert(n.name);
                }
            }
        }
        leaf_nodes = new_leaf_nodes;
    }

    total as i32
}

pub fn part_two() -> i32 {
    let parsed = read_input();
    let end_points = build_trees_2(&parsed);
    let (mut santa, mut us) = (end_points[0].clone(), end_points[1].clone()); // doesn't matter which way around

    let mut seen = HashMap::new();

    let mut steps = 0;
    while let Some(prev) = *santa.orbits {
        santa = prev.clone();
        steps += 1;
        seen.insert(prev.name, steps);
    }

    let mut steps = 0;
    while let Some(prev) = *us.orbits {
        us = prev.clone();
        steps += 1;
        if let Some(x) = seen.get(&prev.name) {
            return steps + x - 2;
            //-2 because it doesn't count one step on each end
        }
    }

    unreachable!("failed to pathfind to santa :(")
}
