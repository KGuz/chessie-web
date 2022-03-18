use super::square::SquareType;
use super::styled_svg::StyledSVGComponent;
use crate::resources::*;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct PieceComponentProps {
    pub typ: SquareType,
}

#[function_component(PieceComponent)]
pub fn piece_component(props: &PieceComponentProps) -> Html {
    let (white, black) = ("white-pice-svg", "black-pice-svg");
    match props.typ {
        SquareType::WPawn   => html! { <StyledSVGComponent class={ white } svg={ PAWN_SVG }/> },
        SquareType::WRook   => html! { <StyledSVGComponent class={ white } svg={ ROOK_SVG }/> },
        SquareType::WKnight => html! { <StyledSVGComponent class={ white } svg={ KNIGHT_SVG }/> },
        SquareType::WBishop => html! { <StyledSVGComponent class={ white } svg={ BISHOP_SVG }/> },
        SquareType::WQueen  => html! { <StyledSVGComponent class={ white } svg={ QUEEN_SVG }/> },
        SquareType::WKing   => html! { <StyledSVGComponent class={ white } svg={ KING_SVG }/> },

        SquareType::BRook   => html! { <StyledSVGComponent class={ black } svg={ ROOK_SVG }/> },
        SquareType::BKnight => html! { <StyledSVGComponent class={ black } svg={ KNIGHT_SVG }/> },
        SquareType::BBishop => html! { <StyledSVGComponent class={ black } svg={ BISHOP_SVG }/> },
        SquareType::BQueen  => html! { <StyledSVGComponent class={ black } svg={ QUEEN_SVG }/> },
        SquareType::BPawn   => html! { <StyledSVGComponent class={ black } svg={ PAWN_SVG }/> },
        SquareType::BKing   => html! { <StyledSVGComponent class={ black } svg={ KING_SVG }/> },
        SquareType::Empty   => html! {},
    }
}
