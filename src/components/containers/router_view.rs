use yew::prelude::*;

#[function_component(RouterView)]
pub fn router_view() -> Html {
    html! {
        <div class="router-view-container">
            <h1>{"RouterView"}</h1>
        </div>
    }
}
