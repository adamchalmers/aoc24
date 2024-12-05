use std::collections::HashMap;
use std::ops::Not;

use anyhow::anyhow;
use anyhow::Result;

fn main() -> Result<()> {
    let input = Input::parse(include_str!("../input"))?;
    let q1 = input.solve_q1();
    println!("Q1: {q1}");
    Ok(())
}

type Update = Vec<u32>;

#[derive(Debug)]
struct Input {
    constraints: Vec<(u32, u32)>,
    updates: Vec<Update>,
}

impl Input {
    fn solve_q1(&self) -> u32 {
        self.updates
            .iter()
            .enumerate()
            .filter_map(|(i, update)| {
                if self.update_is_correct(i) {
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
            .filter(|i| self.update_is_correct(*i))
            .collect()
    }

    fn update_is_correct(&self, update_id: usize) -> bool {
        // Map page numbers to their position in the update list.
        let update: HashMap<u32, usize> = self.updates[update_id]
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
                    // eprintln!("Constraint {page_left}|{page_right} is violated.");
                    // eprintln!("{page_left} is position {l}");
                    // eprintln!("{page_right} is position {r}");
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
}
