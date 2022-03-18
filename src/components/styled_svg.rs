use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct StyledSVGComponentProps {
    pub svg: String,
    pub class: Option<String>,
}

#[function_component(StyledSVGComponent)]
pub fn styled_svg(props: &StyledSVGComponentProps) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.svg);

    let svg = div.first_element_child().unwrap();
    if let Some(class) = &props.class {
        svg.set_attribute("class", class).unwrap();
    }

    Html::VRef(svg.into())
}
