use crate::agent::{SolveOneTask, SolveTwoTask};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_agent::oneshot::use_oneshot_runner;

#[derive(Properties, PartialEq)]
pub struct RunButtonProps {
    pub day: u8,
    pub input: String,
    pub run_callback: Callback<()>,
    pub solve_one_callback: Callback<Result<(String, i64), String>>,
    pub solve_two_callback: Callback<Result<(String, i64), String>>,
}

#[function_component(RunButton)]
pub fn run_button(
    RunButtonProps {
        day,
        input,
        run_callback,
        solve_one_callback,
        solve_two_callback,
    }: &RunButtonProps,
) -> Html {
    let day = *day;
    let input = input.clone();
    let run_cb = run_callback.clone();
    let solve_one_cb = solve_one_callback.clone();
    let solve_two_cb = solve_two_callback.clone();

    let solve1 = use_oneshot_runner::<SolveOneTask>();
    let solve2 = use_oneshot_runner::<SolveTwoTask>();

    let run = move |_: MouseEvent| {
        run_cb.emit(());

        {
            let solve2 = solve2.clone();
            let input = input.clone();
            let solve_two_cb = solve_two_cb.clone();
            spawn_local(async move {
                let res = solve2.run((day, input)).await;
                solve_two_cb.emit(res);
            });
        }
        {
            let solve1 = solve1.clone();
            let input = input.clone();
            let solve_one_cb = solve_one_cb.clone();
            spawn_local(async move {
                let res = solve1.run((day, input)).await;
                solve_one_cb.emit(res);
            });
        }
    };

    let on_click = run.clone();
    html! {
        <button
            onclick={on_click}
            type="button">{"Run"}
        </button>
    }
}
