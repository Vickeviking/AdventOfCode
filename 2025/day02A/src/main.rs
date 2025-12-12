use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, Stderr},
};

// only 0..len valid
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

fn validate(line: String) -> u64 {
    let ranges: Vec<&str> = line.split(',').collect();

    let mut valid_ids = HashSet::<u64>::with_capacity(100000);
    let mut invalid_ids = HashSet::<u64>::with_capacity(100000);

    //use to index id
    let mut buf = [0u8; 20];
    for range in ranges {
        let mut it = range.split('-').map(|s: &str| s.parse::<u64>().unwrap());
        let a = it.next().unwrap();
        let b = it.next().unwrap();

        for id in a..=b {
            // if already valid, or already invalid no action has to be made
            if valid_ids.contains(&id) || invalid_ids.contains(&id) {
                continue;
            }

            let len = digits_u64(id, &mut buf);
            let hlen = len / 2;
            // if number contains an odd amount of digits, it has to be valid
            if len % 2 == 1 {
                valid_ids.insert(id);
                continue;
            }

            // now walk for len/2 , until buf[i] != i + len/2
            let mut is_valid = false;
            for i in 0..hlen {
                if buf[i] != buf[i + hlen] {
                    is_valid = true;
                    break;
                }
            }
            if is_valid {
                valid_ids.insert(id);
            } else {
                invalid_ids.insert(id);
            }
        }
    }

    invalid_ids.iter().sum()
}

fn main() -> io::Result<()> {
    let file = File::open("day02A/src/input.txt")?;
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
        assert!(digits_u64(0, &mut buf) == 0);
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
        let s = "11-22,95-115".to_string();

        assert!(validate(s) == 132)
    }

    #[test]
    fn validate_simple() {
        let s = "11-22,95-115,998-1012".to_string();

        assert!(validate(s) == 1142)
    }

    #[test]
    fn validate_real() {
        let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
            .to_string();

        assert!(validate(s) == 1227775554)
    }
}
