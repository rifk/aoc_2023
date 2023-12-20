use crate::data;
use chrono::Local;
use yew_agent::prelude::*;

#[oneshot]
pub async fn SolveOneTask(input: (u8, String)) -> Result<(String, i64), String> {
    let days = data::day_solvers();
    let day = days
        .iter()
        .find(|d| d.day == input.0)
        .ok_or(format!("missing day {}", input.0))?;
    let start = Local::now();
    (day.solve_one)(&input.1)
        .map(|r| (r, (Local::now() - start).num_milliseconds()))
        .map_err(|e| e.to_string())
}

#[oneshot]
pub async fn SolveTwoTask(input: (u8, String)) -> Result<(String, i64), String> {
    let days = data::day_solvers();
    let day = days
        .iter()
        .find(|d| d.day == input.0)
        .ok_or(format!("missing day {}", input.0))?;
    let start = Local::now();
    (day.solve_two)(&input.1)
        .map(|r| (r, (Local::now() - start).num_milliseconds()))
        .map_err(|e| e.to_string())
}
