#![recursion_limit = "1024"]

use yew::*;
use yew::{html, Component, ComponentLink, Html};

pub mod pages;

mod routes;
use routes::Navigation;

struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <Navigation />
        }
    }
}

fn main() {
    yew::start_app::<Navigation>();
}
