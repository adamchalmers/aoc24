use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use rayon::prelude::*;
use std::ops::Not;

/// Find the middle page from the list of all pages, after they've been
/// ordered according to the given constraints.
fn find_middle_page(all_pages: &HashSet<u32>, constraints: &[(u32, u32)]) -> u32 {
    // First, construct a graph where each X|Y constraint is an edge from X to Y.
    let mut graph = HashMap::default();
    let mut nodes_with_incoming_edges = HashSet::default();
    let mut incoming_edges_for_each_node = HashMap::default();
    for (l, r) in constraints.iter().copied() {
        graph.entry(l).or_insert(Vec::new()).push(r);
        *incoming_edges_for_each_node.entry(r).or_insert(0) += 1;
        nodes_with_incoming_edges.insert(r);
    }
    let mut starts: Vec<u32> = all_pages
        .iter()
        .filter(|num| nodes_with_incoming_edges.contains(num).not())
        .copied()
        .collect();
    drop(nodes_with_incoming_edges);

    // Do a topological sort to find pages in the order they must appear.
    // Stop when we reach the middle page, because that's the only one we care about.
    let mut order = Vec::new();
    let goal = all_pages.len() / 2;
    while order.len() < goal + 1 {
        let curr = starts.pop().unwrap();
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
    order[goal]
}

type Update = Vec<u32>;

#[derive(Debug)]
struct Parsed {
    constraints: Vec<(u32, u32)>,
    updates: Vec<Update>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Parsed {
    let (constraints, updates) = input.split_once("\n\n").unwrap();

    let constraints: Vec<_> = constraints
        .par_lines()
        .map(|line| {
            let (l, r) = line.split_once('|').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let updates: Vec<_> = updates
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    assert!(updates.is_empty().not());
    assert!(constraints.is_empty().not());

    Parsed {
        constraints,
        updates,
    }
}

#[aoc(day5, part2)]
fn solve_q2(parsed: &Parsed) -> u32 {
    parsed
        .updates
        .par_iter()
        .filter_map(|update| {
            if parsed.update_is_correct(update) {
                return None;
            }
            let numbers_in_update: HashSet<_> = update.iter().copied().collect();
            Some(find_middle_page(
                &numbers_in_update,
                &parsed
                    .constraints
                    .iter()
                    .filter(|(l, r)| numbers_in_update.contains(l) && numbers_in_update.contains(r))
                    .copied()
                    .collect::<Vec<_>>(),
            ))
        })
        .sum()
}

#[aoc(day5, part1)]
fn solve_q1(parsed: &Parsed) -> u32 {
    parsed
        .updates
        .par_iter()
        .filter_map(|update| {
            if parsed.update_is_correct(update) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

impl Parsed {
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
        self.constraints.iter().all(|(page_left, page_right)| {
            let pos_left_page = update.get(page_left);
            let pos_right_page = update.get(page_right);
            match (pos_left_page, pos_right_page) {
                (Some(l), Some(r)) => l <= r,
                _ => true,
            }
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

    const INPUT: &str = include_str!("../input/2024/day5.txt");

    #[test]
    fn test_q1() {
        let input = parse(TEST_INPUT);
        assert_eq!(input.updates.len(), 6);
        assert_eq!(input.constraints.len(), 21);

        assert_eq!(input.updates_in_correct_order(), vec![0, 1, 2]);
        assert_eq!(solve_q1(&input), 143);
        let input = parse(INPUT);
        assert_eq!(solve_q1(&input), 5955);
    }

    #[test]
    fn test_q2() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve_q2(&input), 123);
        let input = parse(INPUT);
        assert_eq!(solve_q2(&input), 4030);
    }
}
