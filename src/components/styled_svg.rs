use wasm_bindgen::UnwrapThrowExt;
use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct StyledSVGProps {
    pub svg: String,
    pub class: Option<String>,
}

#[function_component(StyledSVG)]
pub fn styled_svg(props: &StyledSVGProps) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap_throw();
    div.set_inner_html(&props.svg);

    let svg = div.first_element_child().unwrap_throw();
    if let Some(class) = &props.class {
        svg.set_attribute("class", class).unwrap_throw();
    }

    Html::VRef(svg.into())
}
