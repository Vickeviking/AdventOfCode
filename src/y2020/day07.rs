use std::collections::{HashMap, HashSet};

use crate::Solution;

pub struct Day07;

impl Solution for Day07 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day07.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day07.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        7
    }
}

struct BagContent(Option<Vec<(String, usize)>>);

impl BagContent {
    pub fn add(&mut self, color: String, num: usize) {
        if let Some(v) = &mut self.0 {
            v.push((color, num));
        } else {
            self.0 = Some(vec![(color, num)]);
        }
    }
}

fn can_contain_gold(
    color: &str,
    map: &HashMap<String, BagContent>,
    memo: &mut HashSet<String>,
) -> bool {
    // har memo gold? vi har redan varit djupare via rexursion så lämna
    if memo.contains(color) {
        return true;
    }

    let mut contains = false;
    if let Some(bag_content) = map.get(color) {
        if let Some(children) = &bag_content.0 {
            // för varje barn vill vi nu kolla om det är guld, annars fortsätt djupare tills None
            for (child_clr, _) in children {
                if child_clr == "shiny gold" || can_contain_gold(child_clr, map, memo) {
                    memo.insert(color.to_owned());
                    println!("{:?} can contain gold", color);
                    contains = true;
                    break;
                }
            }
        }
    }

    if contains {
        println!("{:?} could contain gold", color);
    } else {
        println!("{:?} could NOT contain gold", color);
    }
    contains
    // kolla om vi har gold som en av våra barn, recursively
}

//TODO: memoiz the ones not containing gold
fn solve_part_a(input: &str) -> usize {
    let mut available: HashSet<String> = HashSet::new();
    let mut map: HashMap<String, BagContent> = HashMap::new();
    let mut contains_goldbag: HashSet<String> = HashSet::new();

    for l in input.lines() {
        let mut iter = l.split("contain");
        let initial_color = iter
            .next()
            .expect("should follow pattern")
            .trim()
            .trim_end_matches('.')
            .trim_end_matches(" bags")
            .trim_end_matches(" bag")
            .to_owned();

        //lets parse its contained bags
        if available.insert(initial_color.clone()) {
            map.insert(initial_color.clone(), BagContent(None));
        };
        let bag_content = map.get_mut(&initial_color).unwrap();

        let bags_iter = iter.next().unwrap().split(',');
        for next in bags_iter {
            let trimmed_next = next
                .trim()
                .trim_end_matches('.')
                .trim_end_matches(" bags")
                .trim_end_matches(" bag");
            if trimmed_next == "no other" {
                break;
            }

            //we have a new bag, parse out num and color
            println!("next: {:?}", trimmed_next);
            let mut cleaned = trimmed_next.split_whitespace(); // num color
            let num = cleaned.next().unwrap().parse::<usize>().unwrap();

            //glue togheter with space
            let color = cleaned.collect::<Vec<_>>().join(" ");
            bag_content.add(color, num);
        }
    }

    // now we for each color in available se if it recursively has gold bag, we use memoization
    // with contains_goldbag
    for color in available.iter() {
        if can_contain_gold(color, &map, &mut contains_goldbag) {
            contains_goldbag.insert(color.clone());
        }
    }

    contains_goldbag.iter().count()
}

fn solve_part_b(input: &str) -> usize {
    let mut map: HashMap<String, BagContent> = HashMap::new();

    for l in input.lines() {
        let mut iter = l.split("contain");
        let initial_color = iter
            .next()
            .expect("should follow pattern")
            .trim()
            .trim_end_matches('.')
            .trim_end_matches(" bags")
            .trim_end_matches(" bag")
            .to_owned();

        //lets parse its contained bags
        let bag_content = map.entry(initial_color.clone()).or_insert(BagContent(None));

        let bags_iter = iter.next().unwrap().split(',');
        for next in bags_iter {
            let trimmed_next = next
                .trim()
                .trim_end_matches('.')
                .trim_end_matches(" bags")
                .trim_end_matches(" bag");
            if trimmed_next == "no other" {
                break;
            }

            //we have a new bag, parse out num and color
            let mut cleaned = trimmed_next.split_whitespace(); // num color
            let num = cleaned.next().unwrap().parse::<usize>().unwrap();

            //glue togheter with space
            let color = cleaned.collect::<Vec<_>>().join(" ");
            bag_content.add(color, num);
        }
    }

    count_inner("shiny gold".to_string(), &map) - 1
}

fn count_inner(strid: String, map: &HashMap<String, BagContent>) -> usize {
    match map.get(&strid).and_then(|b| b.0.as_ref()) {
        Some(bag_content) => {
            let mut sum = 1;
            for (color, count) in bag_content {
                sum += *count * count_inner(color.clone(), map);
            }
            sum
        }
        None => 1, //leaf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(solve_part_a(input), 4);
    }

    #[test]
    fn test_part_b() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(solve_part_b(input), 126);
    }
}

// edge case:
//  'xxx' "bags contain"  [ 'num' 'yyy' "bag.." ] '.' if stop, else ','
//                      or "no other bags."

// we cut out 'xxx' "bags contain"
//      either "no other bags."
//      or [ 'num' 'yyy' "bag.."]
