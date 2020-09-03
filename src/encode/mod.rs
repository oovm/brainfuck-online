use crypto_brainfuck::encode;
use yew::{html, prelude::*, services::ConsoleService, Component, ComponentLink, Html, ShouldRender};

pub enum Event {
    Input(String),
    Clean,
    Copy,
}

pub struct Encoder {
    link: ComponentLink<Self>,
    text: String,
}

impl Component for Encoder {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, text: String::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(text) => {
                self.text = text;
                true
            }
            Event::Clean => {
                self.text = String::new();
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
        let out_text = if self.text.is_empty() { String::new() } else { encode(&self.text).replace('\n', "") };
        let output: Html = html! {
        <div class="form-group">
            <label class="form-label">{"Output Brainfuck Code"}</label>
            <textarea readonly=true rows="10" value=&out_text></textarea>
        </div>
        };
        let bottoms: Html = html! {
        <div class="button-group">
            <button
            class="button danger"
            >
                {crate::header::svg_trash()}
                <span>{"Clear"}</span>
                </button>
            <button
            class="button"
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
