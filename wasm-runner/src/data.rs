use eyre::Result;

#[derive(Clone, PartialEq)]
pub struct Day {
    pub day: u8,
    pub solve_one: fn(&str) -> Result<String>,
    pub solve_two: fn(&str) -> Result<String>,
}
macro_rules! day {
    ($i: literal) => {
        paste::paste! {
            Day {
                day: $i,
                solve_one: [<day $i>]::solve_one,
                solve_two: [<day $i>]::solve_two,
            }
        }
    };
}

pub fn day_numbers() -> Vec<u8> {
    day_solvers().iter().map(|d| d.day).collect()
}

pub fn day_solvers() -> Vec<Day> {
    vec![
        day!(1),
        day!(2),
        day!(3),
        day!(4),
        day!(5),
        day!(6),
        day!(7),
        day!(8),
        day!(9),
        day!(10),
        day!(11),
        day!(12),
        day!(13),
        day!(14),
    ]
}
