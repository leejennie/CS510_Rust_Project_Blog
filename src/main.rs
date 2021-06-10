// This code was based and modified from https://github.com/ahmad2smile/portfolio. The author had a simple way of setting
// up routes that I really liked and a simple route was just needed for this project. The overall routing system was
// used from Yew Router which has code similar to this code.

// The overall code skeleton was taken from the Yew documentation site and Trunk was used to run it. Refer to this page
// to see the intial set up https://yew.rs/docs/getting-started/build-a-sample-app. Running Trunk to build and deploy
// created the dist directory and the addition files within that directory used to run the program.

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
