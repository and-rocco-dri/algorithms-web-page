use yew::prelude::*;
use crate::components::navigation::header_nav::HeaderNav;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <HeaderNav></HeaderNav>
    }
}