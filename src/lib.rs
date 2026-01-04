/// Trait that all Advent of Code solutions must implement
pub trait Solution {
    /// Run part A of the solution
    fn part_a(&self) -> String;

    /// Run part B of the solution
    fn part_b(&self) -> String;

    /// Get the day number
    fn day(&self) -> u8;
}

/// Macro to reduce boilerplate for implementing Solution trait
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        paste::paste! {
            use crate::Solution;

            pub struct [<Day $day>];

            impl Solution for [<Day $day>] {
                fn part_a(&self) -> String {
                    let input = include_str!(concat!("../../inputs/", env!("CARGO_PKG_NAME"), "/day", stringify!([<$day>]), ".txt"));
                    solve_part_a(input).to_string()
                }

                fn part_b(&self) -> String {
                    let input = include_str!(concat!("../../inputs/", env!("CARGO_PKG_NAME"), "/day", stringify!([<$day>]), ".txt"));
                    solve_part_b(input).to_string()
                }

                fn day(&self) -> u8 {
                    $day
                }
            }
        }
    };
}

pub mod y2015;
pub mod y2016;
pub mod y2017;
pub mod y2018;
pub mod y2019;
pub mod y2020;
pub mod y2025;
