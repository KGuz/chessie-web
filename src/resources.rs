use gloo_utils::document;
use yew::{function_component, Html, Properties};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub svg: String,
    pub id: Option<String>,
    pub class: Option<String>,
}

#[function_component(StyledSVG)]
pub fn styled_svg(props: &Props) -> Html {
    let div = document().create_element("div").unwrap();
    div.set_inner_html(&props.svg);

    let svg = div.first_element_child().unwrap();
    if let Some(class) = &props.class {
        svg.set_attribute("class", class).unwrap();
    }
    if let Some(id) = &props.id {
        svg.set_attribute("id", id).unwrap();
    }

    Html::VRef(svg.into())
}

pub const CHESSBOARD_SVG: &'static str = include_str!("../resources/images/Chessboard.svg");

// pub const PAWN_SVG: &'static str = include_str!("../resources/images/Pawn.svg");
