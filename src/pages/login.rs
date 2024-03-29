// The overall code skeleton was taken from the Yew documentation site and Trunk was used to run it. Refer to this page
// to see the intial set up https://yew.rs/docs/getting-started/build-a-sample-app. Running Trunk to build and deploy
// created the dist directory and the addition files within that directory used to run the program.

use yew::prelude::*;

pub struct Login {}

impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Login {}
    }

    fn update(&mut self, _: <Self as yew::html::Component>::Message) -> bool {
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
            <div class="loginpage">
                <div class = "logincontainer">
                    <div class="row">
                        <div class="col-md-6 offset-md-3 col-xs-12">
                            <h1 class="text-xs-center">{ "LOGIN" }</h1>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="email"
                                            placeholder="Email"
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="password"
                                            placeholder="Password"
                                            />
                                    </fieldset>
                                </fieldset>
                                <button
                                        class="btn btn-lg btn-primary btn-block"
                                        type="submit"
                                        disabled=false>
                                        { "Sign in" }
                                </button>
                                <p class="text-s-center">
                                    { "Need an account?" }
                                </p>
                                <p class="text-s-center">
                                    { "Forgot password?" }
                                </p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
