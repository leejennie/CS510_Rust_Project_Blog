// This code was based on and modified from https://github.com/ahmad2smile/portfolio. The author had a simple way of setting
// up routes that I really liked and a simple route was just needed for this project. The overall routing system was
// used from Yew Router which has code similar to this code.

// The overall code skeleton was taken from the Yew documentation site and Trunk was used to run it. Refer to this page
// to see the intial set up https://yew.rs/docs/getting-started/build-a-sample-app. Running Trunk to build and deploy
// created the dist directory and the addition files within that directory used to run the program.

use yew::prelude::*;
use yew_router::{route::Route, service::RouteService, Switch};

use crate::pages::{dashboard::Dashboard, login::Login, recipes::Recipes, signup::Signup};

#[derive(Switch, Clone, Debug)]
pub enum Routes {
    #[to = "/#login"]
    LoginRoute,
    #[to = "/#signup"]
    SignupRoute,
    #[to = "/"]
    DashboardRoute,
    #[to = "/#recipes"]
    RecipesRoute,
}

pub struct Navigation {
    route_service: RouteService<()>,
    route: Route<()>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ChangeRoute(Routes),
    RouteChange(Route<()>),
}

impl Navigation {
    fn change_route(&self, route: Routes) -> Callback<MouseEvent> {
        self.link.callback(move |_| {
            let route = route.clone();
            Msg::ChangeRoute(route)
        })
    }
}

impl Component for Navigation {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let raw_route = route_service.get_route();
        let route = Route::from(raw_route);

        let callback = link.callback(Msg::RouteChange);
        route_service.register_callback(callback);

        Navigation {
            route_service,
            route,
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn update(&mut self, msg: <Self as yew::html::Component>::Message) -> ShouldRender {
        match msg {
            Msg::ChangeRoute(route) => {
                let route_string = match route {
                    Routes::LoginRoute => format!("/#login"),
                    Routes::SignupRoute => format!("/#signup"),
                    Routes::RecipesRoute => format!("/#recipes"),
                    Routes::DashboardRoute => format!("/"),
                };

                self.route_service.set_route(&route_string, ());
                self.route = Route {
                    route: route_string,
                    state: (),
                };
            }
            Msg::RouteChange(route) => self.route = route,
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <nav class="nav-container">
                    <p class="header">{"Recipe Blog"}</p>
                    <ul class="nav flex-column">
                        <li class="nav-item px-5">
                            <a class="nav-link active" onclick=&self.change_route(Routes::DashboardRoute)>
                                {"Home"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" onclick=&self.change_route(Routes::RecipesRoute)>
                                {"Recipes"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" onclick=&self.change_route(Routes::LoginRoute)>
                                {"Login"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" onclick=&self.change_route(Routes::SignupRoute)>
                                {"Signup"}
                            </a>
                        </li>
                    </ul>
                </nav>
                <div class="content">
                    {
                        match Routes::switch(self.route.clone()) {
                            Some(Routes::LoginRoute) => html!{<Login />},
                            Some(Routes::SignupRoute) => html!{<Signup />},
                            Some(Routes::DashboardRoute) => html!{<Dashboard />},
                            Some(Routes::RecipesRoute) => html!{<Recipes />},
                            None => html!{ <div>{"Route Not Found! 404"}</div> }
                        }
                    }
                </div>
            </div>
        }
    }
}
