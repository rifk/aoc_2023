use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DaysListProps {
    pub days: Vec<u8>,
    pub on_click: Callback<u8>,
}

#[function_component(DaysList)]
pub fn days_list(DaysListProps { days, on_click }: &DaysListProps) -> Html {
    //let on_click = on_click.clone();
    let days_list = days
        .iter()
        .map(|&day| {
            let on_day_select = {
                let on_click = on_click.clone();
                Callback::from(move |_| on_click.emit(day))
            };
            html! {
                <a key={day} onclick={on_day_select}>{format!("day {}", day)}</a>
            }
        })
        .collect::<Html>();
    html! {
        <nav>{ days_list }</nav>
    }
}
