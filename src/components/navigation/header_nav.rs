use yew::prelude::*;

#[function_component(HeaderNav)]

pub fn header_nav() -> Html {
    html! {
        <div class="header-container">
            <div class="title-container">
                <div class="title-subcontainer">
                    <div class="icon-container">
                        <img src="images/icon.svg" width="64" height="64" />
                    </div>
                    <div class="title-text-container">
                        <h1>{"ALGViewer"}</h1>
                    </div>
                </div>
            </div>
            <div class="routing-links-container">
                <div>
                    <h1>{"Home"}</h1>
                </div>
                <div>
                    <h1>{"Algorithms"}</h1>
                </div>
            </div>

            
            <div class="settings-container">
                <h1>{"Setting"}</h1>
            </div>
        </div>
    }
}