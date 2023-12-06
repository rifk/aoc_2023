pub mod agent;
mod data;
mod days_list;
mod run_button;
mod runner;

use days_list::DaysList;
use runner::Runner;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let days = data::day_numbers();

    // call backwhen day is selected
    let selected_day = use_state(|| None);
    let on_day_select = {
        let selected_day = selected_day.clone();
        Callback::from(move |day: u8| selected_day.set(Some(day)))
    };
    // start runner for that day
    let runner = selected_day.as_ref().map(|&day| {
        html! {
            <Runner day={day}/>
        }
    });

    html! {
        <>
            <header>
                <h1>{ "AOC 2023 Solver" }</h1>
                <DaysList days={days} on_click={on_day_select.clone()}/>
            </header>
            { for runner }
        </>
    }
}
