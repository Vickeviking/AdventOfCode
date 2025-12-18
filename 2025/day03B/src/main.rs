// ==================================
// joltage rating of a battery 0-9
// each bank contains a list of batteries: 1204910230190
// two is turned on, and the number produced by
// the two in that order is the joltage the bank produces
// the biggest joltage possible is to be found

// bank har N , characters
// hitta högsta siffran index 0-(N-2) (näst sista)
// välj denna siffra som första, gå nu från denna o hitta största siffran
// =================================

use core::num;
use std::{
    char,
    fs::File,
    io::{self, BufRead},
};

// funktion som hittar 'första' största siffran i en sträng mellan index a och b
/// Finds first biggest digit in a string between `start_idx` and `end_idx` and returns its
/// position
///
/// # Arguments
///
/// - start_idx (inclusive)
/// - end_idx (inclusive)
///
/// # Example
///
/// let s = "123456"
/// let digit = highest_digit_in_string(s.to_string(), 0, 3);
/// assert_eq!(digit, (3, 4))
///
/// # Panics
///
/// if string contains non digit chars
///
/// # Returns
///
/// (string_index, largest_digit)
///
fn highest_digit_in_string(s: &str, start_idx: usize, end_idx: usize) -> (usize, u8) {
    let digit_arr: Vec<u8> = s
        .chars()
        .skip(start_idx)
        .take(end_idx - start_idx + 1)
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    let mut biggest_digit: u8 = 0;
    let mut string_index: usize = 0;
    for (index, value) in digit_arr.iter().enumerate() {
        if *value > biggest_digit {
            biggest_digit = *value;
            string_index = index;

            if biggest_digit == 9 {
                break;
            }
        }
    }
    string_index += start_idx;
    (string_index, biggest_digit)
}

fn bank_max_joltage(bank: String) -> u64 {
    let num_batteries = bank.len();
    if num_batteries <= 12 {
        return bank.parse::<u64>().unwrap();
    }

    let mut batteries: Vec<u8> = vec![0u8; 12];

    // 11, 10, 9, ... , 0
    let mut last_idx: usize = 0;
    for i in (0..12).rev() {
        let digit = highest_digit_in_string(&bank, last_idx, num_batteries - i - 1);
        last_idx = digit.0 + 1;
        batteries[11 - i] = digit.1;
    }

    // largest digit 0..N-1
    let mut sum: u64 = 0;

    for b in batteries {
        sum += b as u64;
        sum *= 10;
    }
    sum /= 10;
    sum
}

fn banks_sum_joltage(banks: Vec<String>) -> u64 {
    let mut sum = 0;
    for bank in banks {
        sum += bank_max_joltage(bank);
    }
    sum
}

fn main() -> io::Result<()> {
    let file = File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = io::BufReader::new(file);

    let banks = reader
        .lines()
        .map(|x| x.expect("Valid strings due to proj desc"))
        .collect::<Vec<String>>();

    println!("sum is: {}", banks_sum_joltage(banks));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_digit_in_string() {
        let mut s = "123456";
        let mut digit = highest_digit_in_string(s.to_string(), 0, 3);
        assert_eq!(digit, (3, 4));

        s = "012345678977";
        digit = highest_digit_in_string(s.to_string(), 9, 11);
        assert_eq!(digit, (9, 9));
    }

    #[test]
    fn test_bank_max_voltage() {
        let banks = vec![
            ("987654321111111".to_string(), 987654321111),
            ("811111111111119".to_string(), 811111111119),
            ("234234234234278".to_string(), 434234234278),
            ("818181911112111".to_string(), 888911112111),
        ];

        for bank in banks {
            let max_voltage = bank_max_joltage(bank.0);
            assert_eq!(max_voltage, bank.1);
        }
    }
}
