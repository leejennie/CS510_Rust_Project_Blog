// The overall code skeleton was taken from the Yew documentation site and Trunk was used to run it. Refer to this page
// to see the intial set up https://yew.rs/docs/getting-started/build-a-sample-app. Running Trunk to build and deploy
// created the dist directory and the addition files within that directory used to run the program.

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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="signuppage">
            <div class="signupcontainer">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-10">
                        <h1 class="text-xs-center">{ "SIGN UP" }</h1>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Name"
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Username"
                                    />
                                </fieldset>
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
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Confirm Password"
                                    />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign up" }
                                </button>
                                <p class="text-xs-center">
                                    { "Have an account?" }
                                </p>
                            </fieldset>
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
