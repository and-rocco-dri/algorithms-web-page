use yew::prelude::*;

use crate::components::containers::page_view::PageView;
use crate::components::containers::router_view::RouterView;
use crate::components::navigation::header_nav::HeaderNav;
use crate::components::navigation::footer_nav::FooterNav;

#[component]
pub fn App() -> Html {
    html! {
        <PageView>
            <HeaderNav></HeaderNav>
            <RouterView></RouterView>
            <FooterNav></FooterNav>
        </PageView>
        
    }

}
