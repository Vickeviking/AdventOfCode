use std::collections::{HashMap, HashSet};

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

fn solve_part_a(input: &str) -> u32 {
    // allergens -> possible ingredients
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut ingredient_apperance: HashMap<String, u32> = HashMap::new();

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

        if let Some(list) = allergens {
            for alg in list {
                map.entry(alg)
                    .and_modify(|e| {
                        e.retain(|ingr| ingredients.contains(ingr));
                    })
                    .or_insert(ingredients.to_vec());
            }
        }

        //insert ingredients to set
        for i in ingredients {
            ingredient_apperance
                .entry(i)
                .and_modify(|times| *times += 1)
                .or_insert(1);
        }
    }

    // constraint propagation

    loop {
        let mut changed = false;

        let singles = map
            .iter()
            .filter_map(|(a, v)| {
                if v.len() == 1 {
                    Some((a.clone(), v[0].clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, String)>>();

        let keys: Vec<String> = map.keys().cloned().collect();
        for (allergen, ingred) in singles {
            //remove value from all where allergens != key
            for key in &keys {
                if key == &allergen {
                    continue;
                }

                map.entry(key.clone()).and_modify(|list| {
                    let before = list.len();
                    list.retain(|i| i != &ingred);
                    if before != list.len() {
                        changed = true;
                    }
                });
            }
        }

        if !changed {
            break;
        }
    }

    // Each allergen will map to 1 ingredient, lets pull them out into an array!
    let allergens = map.values().map(|v| v[0].clone()).collect::<Vec<String>>();

    // go through each allergen in set, count each that allergens does not contain
    let mut sum = 0;
    for (i, count) in ingredient_apperance.iter() {
        if !allergens.contains(i) {
            sum += count;
        }
    }

    println!("{:?}", map);

    sum
}

fn solve_part_b(input: &str) -> String {
    // allergens -> possible ingredients
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut ingredient_apperance: HashMap<String, u32> = HashMap::new();

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

        if let Some(list) = allergens {
            for alg in list {
                map.entry(alg)
                    .and_modify(|e| {
                        e.retain(|ingr| ingredients.contains(ingr));
                    })
                    .or_insert(ingredients.to_vec());
            }
        }

        //insert ingredients to set
        for i in ingredients {
            ingredient_apperance
                .entry(i)
                .and_modify(|times| *times += 1)
                .or_insert(1);
        }
    }

    // constraint propagation

    loop {
        let mut changed = false;

        let singles = map
            .iter()
            .filter_map(|(a, v)| {
                if v.len() == 1 {
                    Some((a.clone(), v[0].clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, String)>>();

        let keys: Vec<String> = map.keys().cloned().collect();
        for (allergen, ingred) in singles {
            //remove value from all where allergens != key
            for key in &keys {
                if key == &allergen {
                    continue;
                }

                map.entry(key.clone()).and_modify(|list| {
                    let before = list.len();
                    list.retain(|i| i != &ingred);
                    if before != list.len() {
                        changed = true;
                    }
                });
            }
        }

        if !changed {
            break;
        }
    }

    let mut pairs: Vec<(String, String)> = map
        .iter()
        .map(|(alg, v)| (alg.clone(), v[0].clone()))
        .collect();

    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    pairs
        .into_iter()
        .map(|(_, ingr)| ingr)
        .collect::<Vec<String>>()
        .join(",")
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
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(solve_part_b(input), "mxmxvkd,sqjhc,fvjkl");
    }
}
