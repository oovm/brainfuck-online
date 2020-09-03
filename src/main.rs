#![recursion_limit = "1024"]

mod decode;
mod encode;
mod header;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                {header::view()}
                <header::Encoder/>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
