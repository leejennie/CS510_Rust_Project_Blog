// The overall code skeleton was taken from the Yew documentation site and Trunk was used to run it. Refer to this page 
// to see the intial set up https://yew.rs/docs/getting-started/build-a-sample-app. Running Trunk to build and deploy 
// created the dist directory and the addition files within that directory used to run the program. 

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
            <div class="dashboard">
                <div class="main">{"WELCOME TO THE BEST RECIPE BLOG"}</div>
                <div class="quote">
                { "A place where you will find the recipe you need for that delicious meal you want"}
                </div>
            </div>
        }
    }
}
