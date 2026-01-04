use crate::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day06.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day06.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        6
    }
}

fn solve_part_a(input: &str) -> u32 {
    let mut count = 0;
    for group in input.split("\n\n") {
        let mut group_bitmap: u32 = 0;
        for person in group.lines() {
            let mut person_bitmap: u32 = 0;
            for c in person.chars() {
                let mask = (c as u8 - b'a') as usize;
                person_bitmap ^= 1 << mask;
            }
            group_bitmap |= person_bitmap;
        }
        count += group_bitmap.count_ones();
    }

    count
}

fn solve_part_b(input: &str) -> u32 {
    let mut count = 0;
    for group in input.split("\n\n") {
        let mut group_bitmap: u32 = u32::MAX;
        for person in group.lines() {
            let mut person_bitmap: u32 = 0;
            for c in person.chars() {
                let mask = (c as u8 - b'a') as usize;
                person_bitmap ^= 1 << mask;
            }
            //for each bit in the person_bitmap that is 0, we will make the group_bitmap 0
            group_bitmap &= person_bitmap;
        }
        count += group_bitmap.count_ones();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve_part_a(input), 11);
    }

    #[test]
    fn test_part_b() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve_part_b(input), 6);
    }
}
