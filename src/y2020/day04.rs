use crate::Solution;

pub struct Day04;

impl Solution for Day04 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day04.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day04.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        4
    }
}

fn solve_part_a(input: &str) -> usize {
    let needed_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .split("\n\n")
        .filter(|passport| {
            let fields: Vec<&str> = passport
                .split(&[' ', '\n'][..])
                .map(|field| field.split(':').next().unwrap())
                .collect();
            needed_fields.iter().all(|f| fields.contains(f))
        })
        .count()
}

fn solve_part_b(input: &str) -> usize {
    let needed_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .split("\n\n")
        .filter(|passport| {
            let fields: Vec<(&str, &str)> = passport
                .split(&[' ', '\n'][..])
                .map(|field| {
                    let mut splitted_fields = field.split(':');
                    let a = splitted_fields.next().unwrap();
                    let b = splitted_fields.next().unwrap_or("");
                    (a, b)
                })
                .collect();
            //does each field exist?
            if !needed_fields
                .iter()
                .all(|f| fields.iter().any(|(tag, _)| tag == f))
            {
                return false;
            }
            for f in fields {
                //this is a valid needed field
                match f {
                    ("byr", val) => {
                        let value = match val.parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => return false,
                        };
                        if !(val.len() == 4 && (1920..=2002).contains(&value)) {
                            return false;
                        }
                    }
                    ("iyr", val) => {
                        let value = match val.parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => return false,
                        };
                        if !(val.len() == 4 && (2010..=2020).contains(&value)) {
                            return false;
                        }
                    }
                    ("eyr", val) => {
                        let value = match val.parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => return false,
                        };
                        if !(val.len() == 4 && (2020..=2030).contains(&value)) {
                            return false;
                        }
                    }
                    ("hgt", val) => {
                        let prefix: String = val
                            .chars()
                            .rev()
                            .take(2)
                            .collect::<Vec<_>>() // take last 2 chars reversed
                            .into_iter()
                            .rev() // put them back in order
                            .collect();
                        let rest: String = val.chars().take(val.len() - 2).collect();
                        let value = match rest.parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => return false,
                        };

                        match prefix.as_ref() {
                            "cm" => {
                                if !(150..=193).contains(&value) {
                                    return false;
                                }
                            }
                            "in" => {
                                if !(59..=76).contains(&value) {
                                    return false;
                                }
                            }
                            _ => return false,
                        }
                    }
                    ("hcl", val) => {
                        let prefix: String = val.chars().take(1).collect();
                        let rest: String = val.chars().skip(1).collect();
                        if prefix != "#" {
                            return false;
                        }
                        if !rest.chars().all(|c| c.is_ascii_hexdigit()) || rest.len() != 6 {
                            return false;
                        }
                    }
                    ("ecl", val) => {
                        let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                        if !valid.contains(&val) {
                            return false;
                        }
                    }

                    ("pid", val) => {
                        if val.len() != 9 || !val.chars().all(|c| c.is_ascii_digit()) {
                            return false;
                        }
                    }

                    _ => {}
                }
            }
            true
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hgt:ii

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(solve_part_a(input), 2);
    }

    #[test]
    fn test_part_b() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(solve_part_b(input), 0);
    }
}
