use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Not;

use anyhow::anyhow;
use anyhow::Result;

fn main() -> Result<()> {
    let input = Input::parse(include_str!("../input"))?;
    let q1 = input.solve_q1();
    println!("Q1: {q1}");
    let q2 = input.solve_q2();
    println!("Q2: {q2}");
    Ok(())
}

/// Maps page numbers to their order.
fn topological_ordering(all_numbers: &HashSet<u32>, pairs: &[(u32, u32)]) -> HashMap<u32, usize> {
    let mut graph = HashMap::new();
    let mut nodes_with_incoming_edges = HashSet::new();
    let mut incoming_edges_for_each_node = HashMap::new();
    for (l, r) in pairs.iter().copied() {
        graph.entry(l).or_insert(Vec::new()).push(r);
        *incoming_edges_for_each_node.entry(r).or_insert(0) += 1;
        nodes_with_incoming_edges.insert(r);
    }
    let mut starts: Vec<u32> = all_numbers
        .iter()
        .filter(|num| nodes_with_incoming_edges.contains(num).not())
        .copied()
        .collect();
    drop(nodes_with_incoming_edges);
    let mut order = Vec::new();
    while let Some(curr) = starts.pop() {
        order.push(curr);
        let Some(removed) = graph.remove(&curr) else {
            continue;
        };
        for node_from_curr in removed {
            let num_incoming_edges = incoming_edges_for_each_node
                .get_mut(&node_from_curr)
                .unwrap();
            *num_incoming_edges -= 1;
            if num_incoming_edges == &0 {
                starts.push(node_from_curr);
            }
        }
    }
    order
        .into_iter()
        .enumerate()
        .map(|(i, num)| (num, i))
        .collect()
}

type Update = Vec<u32>;

#[derive(Debug)]
struct Input {
    constraints: Vec<(u32, u32)>,
    updates: Vec<Update>,
}

impl Input {
    fn solve_q2(self) -> u32 {
        self.updates
            .iter()
            .filter_map(|update| {
                if self.update_is_correct(update) {
                    return None::<u32>;
                }
                let mut update = update.to_owned();
                let numbers_in_update: HashSet<_> = update.iter().copied().collect();
                let topsort = topological_ordering(
                    &numbers_in_update,
                    &self
                        .constraints
                        .iter()
                        .filter(|(l, r)| {
                            numbers_in_update.contains(l) && numbers_in_update.contains(r)
                        })
                        .copied()
                        .collect::<Vec<_>>(),
                );
                update.sort_by_key(|num| topsort.get(num).unwrap());
                Some(update[update.len() / 2])
            })
            .sum()
    }

    fn solve_q1(&self) -> u32 {
        self.updates
            .iter()
            .enumerate()
            .filter_map(|(i, update)| {
                if self.update_is_correct(&self.updates[i]) {
                    Some(update[update.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }

    /// Returns indices of updates which are in correct order.
    #[cfg(test)]
    fn updates_in_correct_order(&self) -> Vec<usize> {
        (0..self.updates.len())
            .filter(|i| self.update_is_correct(&self.updates[*i]))
            .collect()
    }

    fn update_is_correct(&self, update: &[u32]) -> bool {
        // Map page numbers to their position in the update list.
        let update: HashMap<u32, usize> = update
            .iter()
            .enumerate()
            .map(|(i, page_num)| (*page_num, i))
            .collect();
        for constraint in &self.constraints {
            let (page_left, page_right) = constraint;
            let pos_left_page = update.get(page_left);
            let pos_right_page = update.get(page_right);
            match (pos_left_page, pos_right_page) {
                (Some(l), Some(r)) if r <= l => {
                    return false;
                }

                _ => continue,
            }
        }
        true
    }

    fn parse(input: &str) -> Result<Self> {
        let (constraints, updates) = input
            .split_once("\n\n")
            .ok_or(anyhow!("no empty line found"))?;

        let constraints: Vec<_> = constraints
            .lines()
            .map(|line| {
                line.split_once('|')
                    .ok_or(anyhow!("no | found on a constraint line"))
                    .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let updates: Vec<_> = updates
            .lines()
            .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
            .collect();

        assert!(updates.is_empty().not());
        assert!(constraints.is_empty().not());

        Ok(Input {
            constraints,
            updates,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_q1() -> Result<()> {
        let input = Input::parse(TEST_INPUT)?;
        assert_eq!(input.updates.len(), 6);
        assert_eq!(input.constraints.len(), 21);

        assert_eq!(input.updates_in_correct_order(), vec![0, 1, 2]);
        assert_eq!(input.solve_q1(), 143);
        Ok(())
    }

    #[test]
    fn test_q2() -> Result<()> {
        let input = Input::parse(TEST_INPUT)?;

        assert_eq!(input.solve_q2(), 123);
        Ok(())
    }
}
