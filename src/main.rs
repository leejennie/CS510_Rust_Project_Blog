#![recursion_limit="256"]

use yew::prelude::*;

pub struct Dashboard {}

impl Component for Dashboard {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Dashboard {} 
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
            <nav class="navbar navbar-expand-lg navbar-dark bg-danger mb-5">
            <a class="navbar-brand"> {"RECIPE BLOG"} </a>
            <div class="collapse navbar-collapse" id="navbarNavDropdown">
              <ul class="navbar-nav">
                <li class="nav-item">
                  <a class="nav-link">{ "Home" }<span class="sr-only"></span></a>
                </li>
                <li class="nav-item">
                  <a class="nav-link">{ "Statistics" }</a>
                </li>
                <li class="nav-item">
                  <a class="nav-link"> { "Sign Up" } </a>
                </li>
                <li class="nav-item">
                  <a class="nav-link"> { "Log In" } </a>
                </li>
              </ul>
            </div>
            </nav>
        }
    }
}

fn main() {
    yew::start_app::<Dashboard>();
}