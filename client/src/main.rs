use gloo::console;
use instant::Instant;
use js_sys::Date;
use speedy::{Readable, Writable};
use std::cell::RefCell;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yew::{html, Component, Context, Html, KeyboardEvent, MouseEvent, NodeRef, TargetCast};

pub enum Msg {
    CreateGoal(String),
}

#[derive(PartialEq, Debug, Readable, Writable)]
pub struct Goal {
    name: String,
    quantity: i64,
    started: i64,
    complete: i64,
}

pub struct RenderTime {
    last_view: RefCell<Option<Instant>>,
    delta_ref: NodeRef,
}

pub struct App {
    all_goals: NodeRef,
    add_goal_button_ref: NodeRef,
    render_time: RenderTime,
    goals: Vec<Goal>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            all_goals: NodeRef::default(),
            add_goal_button_ref: NodeRef::default(),
            render_time: RenderTime {
                last_view: RefCell::default(),
                delta_ref: NodeRef::default(),
            },
            goals: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateGoal(goal) => {
                console::log!(format!("Goal created: {}", goal));
                self.goals.push(Goal {
                    name: goal,
                    quantity: 0,
                    started: 0,
                    complete: 0,
                });
                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        // Apply marquee effect
        /*
                let all_goals = self.all_goals.cast::<HtmlElement>().unwrap();
                for i in 0..all_goals.children().length() {
                    let child = all_goals.children().get_with_index(i).unwrap();
                    let scroll_width = child.scroll_width();
                    let client_width = child.client_width();
                    if scroll_width > client_width {
                        let goal_string = child
                            .clone()
                            .dyn_into::<HtmlInputElement>()
                            .unwrap()
                            .value();
                        console::log!(format!("Goal is too large: {}", goal_string));

                        // Automatically generate style string for marquee
                        // Attempted funny moment but it's not working
                        let hash_class = format!("marquee_input_{}", scroll_width);
                        if !self.marquee_added.contains(&hash_class) {
                            let style_string = format!(
                                r#"
        .{0}:not(:focus) {{
            animation: {0}_keyframes 5s linear infinite;
            overflow: hidden;
            white-space: nowrap;
        }}

        @keyframes {0}_keyframes {{
            0%   {{ text-indent: {1}px }}
            50%  {{ text-indent: -{2}px }}
            100% {{ text-indent: {1}px }}
        }}
                            "#,
                                hash_class, 0, scroll_width
                            );
                            child.class_list().add_1(&hash_class).unwrap();

                            let document = web_sys::window().unwrap().document().unwrap();
                            let style = document.create_element("style").unwrap();
                            style.set_inner_html(&style_string);
                            document.head().unwrap().append_child(&style).unwrap();

                            self.marquee_added.insert(hash_class);
                        }
                    }
                }
                */

        // Show how long it took to render
        let time_after = Instant::now();
        let elapsed_max = time_after - self.render_time.last_view.get_mut().take().unwrap();
        let output = self.render_time.delta_ref.cast::<HtmlElement>().unwrap();
        output.set_inner_text(
            format!("The last rendering took {} ms", elapsed_max.as_millis()).as_str(),
        );
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get how long a render took
        let mut last_view = self.render_time.last_view.borrow_mut();
        if last_view.is_none() {
            *last_view = Some(Instant::now());
        }

        let on_keypress_entry = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::CreateGoal(value))
            } else {
                None
            }
        });

        let add_goal_button_ref = self.add_goal_button_ref.clone();
        let on_goal_add_clicked = ctx.link().batch_callback(move |_e: MouseEvent| {
            let input = add_goal_button_ref.cast::<HtmlInputElement>().unwrap();
            let value = input.value();
            input.set_value("");
            Some(Msg::CreateGoal(value))
        });

        html! {
            <div class="w-full h-full">
                // Display the render time and duration
                <p>
                    <p>{ format!("Rendered: {}", Date::new_0().to_string()) }</p>
                    <p ref={self.render_time.delta_ref.clone()} />
                </p>

                <div class="m-2 flex items-center justify-center">
                    <input class="rounded-l-lg p-4 text-2xl font-bold text-center text-slate-800 border-2 border-slate-500 bg-slate-100" placeholder="Your goal" onkeypress={on_keypress_entry} ref={self.add_goal_button_ref.clone()} />
                    <button class="p-4 rounded-r-lg text-2xl font-bold text-center text-slate-600 border-2 border-slate-500 bg-slate-100 hover:bg-slate-400 hover:text-slate-800" onclick={on_goal_add_clicked}>{"Create goal"}</button>
                </div>
                <ul ref={self.all_goals.clone()}>
                    {for self.goals.iter().map(|goal|
                    html! {
                        <input class="rounded-sm p-2 border-2 text-2xl font-bold text-center text-slate-800 border-slate-500 bg-slate-200" value={goal.name.clone()} />
                    }
                    )}
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
