use yew::prelude::*;

pub struct Signup {}

impl Component for Signup {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Signup {}
    }

    fn update(&mut self, _: <Self as yew::html::Component>::Message) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <div class="contact">
                <h2 class="heading">{"Contact Me!"}</h2>
                <h3 class = "abc">{"hello"}</h3>
            </div>
        }
    }
}