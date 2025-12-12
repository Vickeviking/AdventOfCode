use std::{
    fs::File,
    io::{self, BufRead},
};

pub struct dial_instruction {
    left: bool,
    steps: u16,
}

impl dial_instruction {
    pub fn new(s: String) -> Self {
        Self::from_string(s)
    }

    // String cant be empty
    fn from_string(s: String) -> Self {
        let mut char_iter = s.chars();
        let left = match char_iter.next().expect("atleast one char") {
            'L' => true,
            'R' => false,
            _ => panic!("Something went very wrong"),
        };
        let steps: u16 = char_iter.collect::<String>().parse().unwrap();

        dial_instruction { left, steps }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("day01A/src/input.txt")?;
    let reader = io::BufReader::new(file);

    // Amount of times the dial points at 0
    let mut zeroes: u32 = 0;
    let mut dial: i16 = 50;

    for line in reader.lines() {
        let ins = dial_instruction::from_string(line.unwrap());
        dial = match ins.left {
            true => (dial - ins.steps as i16) % 100,
            false => (dial + ins.steps as i16) % 100,
        };

        if dial == 0 {
            zeroes += 1;
        }
    }

    println!("the dial ended up on 0 {:?} times!", zeroes);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_instruction() {
        let d = dial_instruction::new("L12".to_string());
        assert!(d.left);
        assert_eq!(d.steps, 12);
    }

    #[test]
    fn test_right_instruction() {
        let d = dial_instruction::new("R5".to_string());
        assert!(!d.left);
        assert_eq!(d.steps, 5);
    }

    #[test]
    fn test_run() {
        let instructions: Vec<String> = vec![
            "L68".into(),
            "L30".into(),
            "R48".into(),
            "L5".into(),
            "R60".into(),
            "L55".into(),
            "L1".into(),
            "L99".into(),
            "R14".into(),
            "L82".into(),
        ];

        let mut zeroes: u32 = 0;
        let mut dial: i16 = 50;

        for line in instructions {
            let ins = dial_instruction::from_string(line);
            dial = match ins.left {
                true => (dial - ins.steps as i16) % 100,
                false => (dial + ins.steps as i16) % 100,
            };

            if dial == 0 {
                zeroes += 1;
            }
        }
    }
}
