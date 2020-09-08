use crypto_brainfuck::encode;
use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub enum Event {
    Input(String),
    Clean,
    Copy,
}

pub struct Encoder {
    link: ComponentLink<Self>,
    text: String,
    output: String,
}

impl Component for Encoder {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, text: String::new(), output: String::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(text) => {
                if text.is_empty() {
                    self.text = String::new();
                    self.output = String::new();
                }
                else {
                    // if let Some(c) = self.text.chars().last() {
                    // if c == text.chars().last().unwrap() {
                    // if text.len() - 1 == self.text.len() {
                    // self.text.push(c);
                    // self.output.push('.');
                    // return true;
                    // }
                    // else if text.len() + 1 == self.text.len() {
                    // self.text.pop();
                    // self.output.pop();
                    // return true;
                    // }
                    // }
                    // }
                    self.text = text;
                    // FixMe: Rendering is blocked here
                    // TODO: use async encode
                    self.output = encode(&self.text).replace('\n', "")
                };
                true
            }
            Event::Clean => {
                self.text = String::new();
                self.output = String::new();
                true
            }
            Event::Copy => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let input: Html = html! {
        <div class="form-group">
            <label class="form-label">{"Input String"}</label>
            <textarea
            rows="10"
            oninput=self.link.callback(|input: InputData| Event::Input(input.value))
            value=&self.text
            ></textarea>
        </div>
        };
        let output: Html = html! {
        <div class="form-group">
            <label class="form-label">{"Output Brainfuck Code"}</label>
            <textarea readonly=true rows="10" value=&self.output></textarea>
        </div>
        };
        let bottoms: Html = html! {
        <div class="button-group">
            <button
            class="button danger"
            onclick=self.link.callback(|_| Event::Clean)
            >
                {crate::header::svg_trash()}
                <span>{"Clear"}</span>
                </button>
            <button
            class="button"
            onclick=self.link.callback(|_| Event::Copy)
            >
                {crate::header::svg_clipboard()}
                <span>{"Copy"}</span>
            </button>
        </div>
        };
        html! {
            <div id="encoder">
            {input}
            {output}
            {bottoms}
            </div>
        }
    }
}
