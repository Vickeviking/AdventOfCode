use crate::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2025/day02.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2025/day02.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        2
    }
}

#[inline]
fn digits_u64(mut n: u64, out: &mut [u8; 20]) -> usize {
    if n == 0 {
        out[0] = 0;
        return 0;
    }

    let mut len = 0;
    while n > 0 {
        out[len] = (n % 10) as u8;
        n /= 10;
        len += 1;
    }

    out[..len].reverse();
    len
}

fn solve_part_a(input: &str) -> u64 {
    let line = input.lines().next().unwrap();
    let ranges: Vec<&str> = line.split(',').collect();
    let mut invalid_id_sum: u64 = 0;

    let mut buf = [0u8; 20];
    for range in ranges {
        let mut it = range.split('-').map(|s: &str| s.parse::<u64>().unwrap());
        let a = it.next().unwrap();
        let b = it.next().unwrap();

        for id in a..=b {
            let len = digits_u64(id, &mut buf);
            let hlen = len / 2;
            
            if len % 2 == 1 {
                continue;
            }

            let mut is_valid = false;
            for i in 0..hlen {
                if buf[i] != buf[i + hlen] {
                    is_valid = true;
                    break;
                }
            }

            if !is_valid {
                invalid_id_sum += id;
            }
        }
    }

    invalid_id_sum
}

fn solve_part_b(input: &str) -> u64 {
    let line = input.lines().next().unwrap();
    let ranges: Vec<&str> = line.split(',').collect();
    let mut invalid_id_sum: u64 = 0;

    let mut buf = [0u8; 20];
    for range in ranges {
        let mut it = range.split('-').map(|s: &str| s.parse::<u64>().unwrap());
        let a = it.next().unwrap();
        let b = it.next().unwrap();

        for id in a..=b {
            let len = digits_u64(id, &mut buf);
            
            for win_num in 2..=len {
                if len % win_num == 0 {
                    let mut fits = true;
                    let win_len = len / win_num;

                    for index in 0..win_len {
                        for win_offset in 1..win_num {
                            if buf[index] != buf[(win_len * win_offset) + index] {
                                fits = false;
                                break;
                            }
                        }
                    }

                    if fits {
                        invalid_id_sum += id;
                        break;
                    }
                }
            }
        }
    }

    invalid_id_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_u64() {
        let mut buf = [0u8; 20];
        assert_eq!(digits_u64(12, &mut buf), 2);
        assert_eq!(digits_u64(123, &mut buf), 3);
        assert_eq!(digits_u64(1234, &mut buf), 4);

        digits_u64(123456, &mut buf);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);
        assert_eq!(buf[3], 4);
        assert_eq!(buf[4], 5);
        assert_eq!(buf[5], 6);
    }

    #[test]
    fn test_validate_simple() {
        let s = "11-22,95-115";
        assert_eq!(solve_part_a(s), 132);
    }
}
