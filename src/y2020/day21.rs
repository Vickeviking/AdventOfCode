use std::collections::HashMap;

use crate::Solution;

pub struct Day21;

impl Solution for Day21 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day21.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day21.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        21
    }
}

fn solve_part_a(input: &str) -> i32 {
    //allergens -> ingredients
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for l in input.lines() {
        let mut parts = l.split("(contains");
        let ingredients: Vec<String> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let allergens: Option<Vec<String>> = parts.next().map(|s| {
            s.trim_end_matches(')')
                .split(',')
                .map(|allerg| allerg.trim().to_string())
                .collect()
        });

        match allergens {
            Some(list) => {
                // nu vill vi slå upp på varje allergen
                for alg in list {
                    map.entry(alg)
                        .and_modify(|e| {
                            e.retain(|ingr| ingredients.contains(ingr));
                        })
                        .or_insert(ingredients.iter().map(|s| s.to_string()).collect());
                }
            }
            None => continue,
        }
    }

    // now map will map each allergen to a ingredient,
    // so ingredients not apart of map is free,
    //
    // make a hashset , and add udirng  init phase, iter over set and se if apart over values from
    // map

    0
}

fn solve_part_b(_input: &str) -> i32 {
    // TODO: Implement part B
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(solve_part_a(input), 5);
    }

    #[test]
    fn test_part_b() {
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }
}
