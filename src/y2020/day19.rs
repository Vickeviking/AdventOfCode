use std::collections::HashMap;

use crate::Solution;

pub struct Day19;

// Valid messages obey rules
//
// Input:
// Rules and Messages

// Rules, numbered and build upon eachother, seperated by ':' ie 123123: rule

// Ruletypes:
//
// - single character match "a"
// - sub rules, 0: 1 2 , must match rule 1 , where the text after the matching part must match r 2
// - list of subrules, must match one of the sub-rules i.ex
//   2: 1 3 | 3 1 , for 2 to match, either 1 -> 3 , or 3 -> 1 must match

impl Solution for Day19 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day19.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day19.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        19
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Rule {
    BaseRule(char),
    SubRules(Vec<Vec<u16>>),
}

fn solve_part_a(input: &str) -> i32 {
    //binds rule number to rule
    let mut map_rules: HashMap<u16, Rule> = HashMap::new();

    let mut parts = input.split("\n\n");
    let rules_input: &str = parts.next().unwrap();
    let messages: Vec<&str> = parts.next().unwrap().lines().collect();

    for line in rules_input.lines() {
        let mut split = line.split(':');
        let rule_num = split.next().unwrap().parse::<u16>().unwrap();
        let rule_body = split.next().unwrap();

        if rule_body.contains('"') {
            //this is a char
            let c = rule_body.trim().trim_matches('"').chars().next().unwrap();
            map_rules.insert(rule_num, Rule::BaseRule(c));
        } else {
            let sub_rules: Vec<Vec<u16>> = rule_body
                .split('|')
                .map(|sub_rule| {
                    sub_rule
                        .split_whitespace()
                        .map(|n| n.parse::<u16>().unwrap())
                        .collect()
                })
                .collect();

            map_rules.insert(rule_num, Rule::SubRules(sub_rules));
        }
    }

    let rule_zero = map_rules.get(&0).unwrap();

    /// Returns true if the rule matches the message starting at index, and consumes the whole message
    fn evaluate(
        rule: &Rule,
        map_rules: &HashMap<u16, Rule>,
        message: &str,
        index: usize,
    ) -> Vec<usize> {
        match rule {
            Rule::BaseRule(c) => {
                // If the char at index matches, return the next index
                if message.chars().nth(index) == Some(*c) {
                    vec![index + 1]
                } else {
                    vec![]
                }
            }
            Rule::SubRules(subrules) => {
                let mut results = vec![];

                for subrule in subrules {
                    // Start with current index
                    let mut indices = vec![index];

                    for &rule_num in subrule {
                        // For each index so far, try matching the next rule
                        indices = indices
                            .iter()
                            .flat_map(|&i| {
                                evaluate(map_rules.get(&rule_num).unwrap(), map_rules, message, i)
                            })
                            .collect();

                        // abort early if no matches left
                        if indices.is_empty() {
                            break;
                        }
                    }

                    results.extend(indices);
                }

                results
            }
        }
    }

    let mut nmbr_of_valid = 0;
    for m in messages {
        if evaluate(rule_zero, &map_rules, &m, 0).contains(&m.len()) {
            nmbr_of_valid += 1;
        }
    }

    nmbr_of_valid
}

fn solve_part_b(input: &str) -> i32 {
    //binds rule number to rule
    let mut map_rules: HashMap<u16, Rule> = HashMap::new();

    let mut parts = input.split("\n\n");
    let rules_input: &str = parts.next().unwrap();
    let messages: Vec<&str> = parts.next().unwrap().lines().collect();

    for line in rules_input.lines() {
        let mut modified_line = line.to_string();
        if line == "8: 42" {
            modified_line = "8: 42 | 42 8".to_string();
        } else if line == "11: 42 31" {
            modified_line = "11: 42 31 | 42 11 31".to_string();
        }

        let mut split = modified_line.split(':');
        let rule_num = split.next().unwrap().parse::<u16>().unwrap();
        let rule_body = split.next().unwrap();

        if rule_body.contains('"') {
            //this is a char
            let c = rule_body.trim().trim_matches('"').chars().next().unwrap();
            map_rules.insert(rule_num, Rule::BaseRule(c));
        } else {
            let sub_rules: Vec<Vec<u16>> = rule_body
                .split('|')
                .map(|sub_rule| {
                    sub_rule
                        .split_whitespace()
                        .map(|n| n.parse::<u16>().unwrap())
                        .collect()
                })
                .collect();

            map_rules.insert(rule_num, Rule::SubRules(sub_rules));
        }
    }

    let rule_zero = map_rules.get(&0).unwrap();

    /// Returns true if the rule matches the message starting at index, and consumes the whole message
    fn evaluate(
        rule: &Rule,
        map_rules: &HashMap<u16, Rule>,
        message: &str,
        index: usize,
    ) -> Vec<usize> {
        match rule {
            Rule::BaseRule(c) => {
                // If the char at index matches, return the next index
                if message.chars().nth(index) == Some(*c) {
                    vec![index + 1]
                } else {
                    vec![]
                }
            }
            Rule::SubRules(subrules) => {
                let mut results = vec![];

                for subrule in subrules {
                    // Start with current index
                    let mut indices = vec![index];

                    for &rule_num in subrule {
                        // For each index so far, try matching the next rule
                        indices = indices
                            .iter()
                            .flat_map(|&i| {
                                evaluate(map_rules.get(&rule_num).unwrap(), map_rules, message, i)
                            })
                            .collect();

                        // abort early if no matches left
                        if indices.is_empty() {
                            break;
                        }
                    }

                    results.extend(indices);
                }

                results
            }
        }
    }

    let mut nmbr_of_valid = 0;
    for m in messages {
        if evaluate(rule_zero, &map_rules, &m, 0).contains(&m.len()) {
            nmbr_of_valid += 1;
        }
    }

    nmbr_of_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"}
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        assert_eq!(solve_part_a(input), 2);
    }

    #[test]
    fn test_part_b() {
        let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        assert_eq!(solve_part_b(input), 12);
    }
}
