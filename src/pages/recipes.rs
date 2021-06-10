// The overall code skeleton was taken from the Yew documentation site and Trunk was used to run it. Refer to this page
// to see the intial set up https://yew.rs/docs/getting-started/build-a-sample-app. Running Trunk to build and deploy
// created the dist directory and the addition files within that directory used to run the program.

use yew::prelude::*;

pub struct Recipes {}

impl Component for Recipes {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Recipes {}
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
            <div class = "recipe">
                <li class = "todo">
                    { "Need drop down menu with different types of recipes" }
                </li>
                <li class = "todo">
                    { "Need to add images" }
                </li>
                <li class = "todo">
                    { "Need ingredients section" }
                </li>
                <li class = "todo">
                    { "Need recipe section" }
                </li>
            </div>
        }
    }
}
