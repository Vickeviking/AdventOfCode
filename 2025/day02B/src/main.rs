use std::{
    fs::File,
    io::{self, BufRead},
};

// only 0..len valid
#[inline]
fn digits_u64(mut n: u64, out: &mut [u8; 20]) -> usize {
    if n == 0 {
        out[0] = 0;
        return 1;
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

fn validate(line: String) -> u64 {
    let ranges: Vec<&str> = line.split(',').collect();
    let mut invalid_id_sum: u64 = 0;

    //use to index id
    let mut buf = [0u8; 20];
    for range in ranges {
        let mut it = range.split('-').map(|s: &str| s.parse::<u64>().unwrap());
        let a = it.next().unwrap();
        let b = it.next().unwrap();

        for id in a..=b {
            let len = digits_u64(id, &mut buf);
            // min number of windows can be 2 up to len,
            // resulting in n/win_n window length
            for win_num in 2..=len {
                if len.is_multiple_of(win_num) {
                    // only if win len is a divisor can i tbe valid
                    let mut fits = true;
                    //check all windows against the first one for each index
                    let win_len = len / win_num;

                    for index in 0..win_len {
                        for win_offset in 1..win_num {
                            //check all windows against the first one
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

fn main() -> io::Result<()> {
    let file = File::open("day02B/src/input.txt")?;
    let reader = io::BufReader::new(file);

    let line: String = reader.lines().next().unwrap()?;

    println!("sum is: {}", validate(line));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_u64() {
        let mut buf = [0u8; 20];
        assert!(digits_u64(0, &mut buf) == 1);
        assert!(digits_u64(12, &mut buf) == 2);
        assert!(digits_u64(123, &mut buf) == 3);
        assert!(digits_u64(1234, &mut buf) == 4);

        digits_u64(123456, &mut buf);
        print!("{:?}", buf);
        assert!(buf[0] == 1);
        assert!(buf[1] == 2);
        assert!(buf[2] == 3);
        assert!(buf[3] == 4);
        assert!(buf[4] == 5);
        assert!(buf[5] == 6);
    }

    #[test]
    fn validate_super_simple() {
        let s = "2121212118-2121212124".to_string();

        assert!(validate(s) == 2121212121)
    }
}
