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
    let file = File::open("day01B/src/input.txt")?;
    let reader = io::BufReader::new(file);

    // Amount of times the dial points at 0
    let mut zeroes: u16 = 0;
    let mut dial: i16 = 50;

    for line in reader.lines() {
        // dial is never 100, it is 0, [0..99]

        let ins = dial_instruction::from_string(line.unwrap());

        // add full cycles,
        // normalize steps, 116 => 16, 323 => 23
        // add cycles as times passed 0,
        let cycles = ins.steps / 100;
        let normalized_steps: i16 = (ins.steps % 100) as i16;
        zeroes += cycles;

        if ins.left {
            // if dial == 0 , add and move on
            if dial == 0 {
                dial = 100 - normalized_steps;
                continue;
            }

            // now we have: 0 < steps left < 100 ,
            // if these steps take dial <= 0 add zeroes,
            // normalize new dial, -5 => 95, 5 => 5, 0 => 0

            let new_dial: i16 = dial - normalized_steps;
            if new_dial <= 0 {
                zeroes += 1;
            }
            dial = new_dial.rem_euclid(100);
        } else {
            // if dial == 0 , add and move on
            if dial == 0 {
                dial = normalized_steps;
                continue;
            }
            // now we have: 0 < steps left < 100 ,
            // if these steps take dial >= 100 add zeroes,
            // normalize new dial, 105 => 5, 5 => 5, 0 => 0 , 100 => 0

            let new_dial = dial + normalized_steps;
            if new_dial >= 100 {
                zeroes += 1;
            }
            dial = new_dial.rem_euclid(100);
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

        let mut zeroes: u16 = 0;
        let mut dial: i16 = 50;

        for line in instructions {
            let ins = dial_instruction::from_string(line);
            // add full cycles,
            // normalize steps, 116 => 16, 323 => 23
            // add cycles as times passed 0,
            let cycles = ins.steps / 100;
            let normalized_steps: i16 = (ins.steps % 100) as i16;
            zeroes += cycles;

            if ins.left {
                // if dial == 0 , add and move on
                if dial == 0 {
                    dial = 100 - normalized_steps;
                    continue;
                }

                // now we have: 0 < steps left < 100 ,
                // if these steps take dial <= 0 add zeroes,
                // normalize new dial, -5 => 95, 5 => 5, 0 => 0

                let new_dial: i16 = dial - normalized_steps;
                if new_dial <= 0 {
                    zeroes += 1;
                }
                dial = new_dial.rem_euclid(100);
            } else {
                // if dial == 0 , add and move on
                if dial == 0 {
                    dial = normalized_steps;
                    continue;
                }
                // now we have: 0 < steps left < 100 ,
                // if these steps take dial >= 100 add zeroes,
                // normalize new dial, 105 => 5, 5 => 5, 0 => 0 , 100 => 0

                let new_dial = dial + normalized_steps;
                if new_dial >= 100 {
                    zeroes += 1;
                }
                dial = new_dial.rem_euclid(100);
            }
        }

        println!("{}", zeroes);
        assert!(zeroes == 6)
    }
}
