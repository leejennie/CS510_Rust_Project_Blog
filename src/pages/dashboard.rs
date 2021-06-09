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
                <div class="welcome">{"WELCOME TO THE BEST RECIPE BLOG"}</div>
            </div>
        }
    }
}