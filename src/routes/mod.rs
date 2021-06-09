use yew::prelude::*;
use yew_router::{route::Route, service::RouteService, Switch};

use crate::pages::{
    login::Login, signup::Signup, dashboard::Dashboard,
};

#[derive(Switch, Clone, Debug)]
pub enum Routes {
    #[to = "/#login"]
    LoginRoute,
    #[to = "/#signup"]
    SignupRoute,
    #[to = "/"]
    DashboardRoute,
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
                        <li class="nav-item px-3">
                            <a class="nav-link active" onclick=&self.change_route(Routes::DashboardRoute)>
                                {"Home"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" onclick=&self.change_route(Routes::LoginRoute) >
                                {"Login"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" onclick=&self.change_route(Routes::SignupRoute) >
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
                            None => html!{ <div>{"Route Not Found! 404"}</div> }
                        }
                    }
                </div>
            </div>
        }
    }
}