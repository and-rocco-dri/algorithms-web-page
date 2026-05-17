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
                        <h1 class="title-text">{"ALGOViewer"}</h1>
                    </div>
                </div>
            </div>
            <div class="side-container">
                <div class="left-padding-container">
                    <div style="padding: 32px;"></div>
                </div>
                <div class="routing-links-container"> 
                    <div >
                        <h3>{"Home"}</h3>
                    </div>
                    <div style="padding-left: 16px;">
                        <h3>{"Algorithms"}</h3>
                    </div>
                    <div style="padding-left: 16px;">
                        <h3>{"About"}</h3>
                    </div>
                </div>
                
                <div class="settings-container">
                    <img src="images/settings-icon.svg" width="32" height="32" />
                </div>
            </div>

            
           
        </div>
    }
}