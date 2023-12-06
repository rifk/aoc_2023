use crate::agent::{SolveOneTask, SolveTwoTask};
use crate::run_button::RunButton;
use web_sys::{HtmlTextAreaElement, InputEvent};
use yew::prelude::*;
use yew_agent::oneshot::OneshotProvider;

#[derive(Debug)]
enum Output {
    Calculating(usize),
    Error(String),
    Solution(String),
}
impl Output {
    fn to_html(&self) -> Html {
        match self {
            Self::Calculating(count) => html! {<>{"calculating"}{vec!['.'; *count]}</>},
            Self::Error(err) => html! {<>{"ERROR - "}{err}</>},
            Self::Solution(sol) => html! {{sol}},
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    InputUpdate(String),
    Run,
    Tick,
    OkOne(String),
    ErrOne(String),
    OkTwo(String),
    ErrTwo(String),
}

#[derive(Properties, PartialEq)]
pub struct RunnerProps {
    pub day: u8,
}

pub struct Runner {
    input: String,
    output: Option<(Output, Output)>,
}

impl Component for Runner {
    type Message = Msg;
    type Properties = RunnerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::default(),
            output: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let day = ctx.props().day;

        let run_callback = {
            let link = ctx.link().clone();
            Callback::from(move |_| {
                link.send_message(Msg::Run);
            })
        };

        let solve_one_callback = {
            let link = ctx.link().clone();
            Callback::from(move |res: Result<String, String>| {
                link.send_message(res.map(Msg::OkOne).unwrap_or_else(Msg::ErrOne));
            })
        };

        let solve_two_callback = {
            let link = ctx.link().clone();
            Callback::from(move |res: Result<String, String>| {
                link.send_message(res.map(Msg::OkTwo).unwrap_or_else(Msg::ErrTwo));
            })
        };

        html! {
            <div>
                <h3>{ format!("Day {}", day) }</h3>
                <OneshotProvider<SolveOneTask> path="/aoc_2023/worker1.js">
                    <OneshotProvider<SolveTwoTask> path="/aoc_2023/worker2.js">
                        <RunButton
                            day={day}
                            input={self.input.clone()}
                            run_callback={run_callback}
                            solve_one_callback={solve_one_callback}
                            solve_two_callback={solve_two_callback}/>
                    </OneshotProvider<SolveTwoTask>>
                </OneshotProvider<SolveOneTask>>
                if let Some((o1, o2)) = &self.output {
                    <p>
                        <b>{"Part One: "}</b>{o1.to_html()}
                        <br />
                        <b>{"Part Two: "}</b>{o2.to_html()}
                    </p>
                }
                <p>{"Enter puzzle input"}</p>
                <textarea
                    value={self.input.clone()}
                    oninput={ctx.link().callback(|event: InputEvent| {
                        let input: HtmlTextAreaElement = event.target_unchecked_into();
                        Msg::InputUpdate(input.value())
                    })}
                    name="puzzle input">
                </textarea>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputUpdate(input) => {
                self.input = input;
                true
            }
            Msg::Run => {
                self.output = Some((Output::Calculating(1), Output::Calculating(1)));
                ctx.link().send_future(async move {
                    yew::platform::time::sleep(std::time::Duration::from_secs(1)).await;
                    Msg::Tick
                });

                true
            }
            Msg::Tick => {
                let mut updated = false;
                if let Some((o1, o2)) = self.output.as_mut() {
                    if let Output::Calculating(c) = o1 {
                        *c = (*c % 3) + 1;
                        updated = true;
                    }
                    if let Output::Calculating(c) = o2 {
                        *c = (*c % 3) + 1;
                        updated = true;
                    }
                }

                if updated {
                    ctx.link().send_future(async move {
                        yew::platform::time::sleep(std::time::Duration::from_secs(1)).await;
                        Msg::Tick
                    });
                }

                updated
            }
            Msg::OkOne(sol) => {
                if let Some((o1, _)) = self.output.as_mut() {
                    *o1 = Output::Solution(sol);
                    true
                } else {
                    false
                }
            }
            Msg::ErrOne(err) => {
                if let Some((o1, _)) = self.output.as_mut() {
                    *o1 = Output::Error(err);
                    true
                } else {
                    false
                }
            }
            Msg::OkTwo(sol) => {
                if let Some((_, o2)) = self.output.as_mut() {
                    *o2 = Output::Solution(sol);
                    true
                } else {
                    false
                }
            }
            Msg::ErrTwo(err) => {
                if let Some((_, o2)) = self.output.as_mut() {
                    *o2 = Output::Error(err);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().day == old_props.day {
            false
        } else {
            self.input = String::default();
            self.output = None;
            true
        }
    }
}
