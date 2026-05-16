use yew::prelude::*;
use yew_router::{Routable, BrowserRouter, Switch};

use crate::components::home::Home;

#[component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render = {switch} />
        </BrowserRouter>
    }

}

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{<Home />}
    }
}

