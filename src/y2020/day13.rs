use crate::Solution;

pub struct Day13;

impl Solution for Day13 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day13.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day13.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        13
    }
}

fn solve_part_a(input: &str) -> u32 {
    let mut iter = input.lines();
    let arrival_time = iter.next().unwrap().parse::<u32>().unwrap();
    let busses = iter
        .next()
        .unwrap()
        .split(',')
        .filter(|&str| str != "x")
        .map(|num| num.parse::<u32>().unwrap());

    // depart_time, wait, id
    let mut earliest_bus: Option<(u32, u32, u32)> = None;
    for buss_id in busses {
        // buss_id * i >= arrival_time
        // i >= arrival_time/buss_id
        // i = (arrival_time/ buss_id).ceil,

        let wait = (buss_id - arrival_time % buss_id) % buss_id;
        let depart_time = arrival_time + wait;

        earliest_bus = match earliest_bus {
            None => Some((depart_time, depart_time - arrival_time, buss_id)),
            Some((dt, _, _)) => {
                if depart_time < dt {
                    // found earlier bus
                    Some((depart_time, depart_time - arrival_time, buss_id))
                } else {
                    earliest_bus
                }
            }
        }
    }
    let inner = earliest_bus.unwrap();
    inner.1 * inner.2
}

fn solve_part_b(input: &str) -> usize {
    let mut iter = input.lines();
    let busses: Vec<(usize, u32)> = iter
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(idx, num)| {
            if num != "x" {
                Some((idx, num.parse::<u32>().unwrap()))
            } else {
                None
            }
        })
        .collect();

    let mut t = 0;
    let mut step = 1;
    for (offset, buss_id) in &busses {
        // tills vi hittar första bussen
        while (t + offset) % *buss_id as usize != 0 {
            t += step; // kolla nästa steg
        }
        //buss worked , add its cycle as a factor of stepsize
        step *= *buss_id as usize
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "939
7,13,x,x,59,x,31,19";
        assert_eq!(solve_part_a(input), 295);
    }

    #[test]
    fn test_part_b() {
        let input = "939
7,13,x,x,59,x,31,19";
        assert_eq!(solve_part_b(input), 1068781);
    }
}
