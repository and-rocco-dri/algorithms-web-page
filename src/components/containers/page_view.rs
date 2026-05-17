use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageViewProps {
    pub children: Html,
}

#[function_component(PageView)]
pub fn page_view(props: &PageViewProps) -> Html {
    html! {
        <div>
            {props.children.clone()}
        </div>
    }
}