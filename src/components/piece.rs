use super::square::SquareType;
use super::styled_svg::StyledSVG;
use crate::resources::*;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct PieceProps {
    pub typ: SquareType,
}

#[function_component(PieceComponent)]
pub fn piece_component(props: &PieceProps) -> Html {
    let (white, black) = ("white-pice-svg", "black-pice-svg");
    match props.typ {
        SquareType::WPawn   => html! { <StyledSVG class={ white } svg={ PAWN_SVG }/> },
        SquareType::WRook   => html! { <StyledSVG class={ white } svg={ ROOK_SVG }/> },
        SquareType::WKnight => html! { <StyledSVG class={ white } svg={ KNIGHT_SVG }/> },
        SquareType::WBishop => html! { <StyledSVG class={ white } svg={ BISHOP_SVG }/> },
        SquareType::WQueen  => html! { <StyledSVG class={ white } svg={ QUEEN_SVG }/> },
        SquareType::WKing   => html! { <StyledSVG class={ white } svg={ KING_SVG }/> },

        SquareType::BRook   => html! { <StyledSVG class={ black } svg={ ROOK_SVG }/> },
        SquareType::BKnight => html! { <StyledSVG class={ black } svg={ KNIGHT_SVG }/> },
        SquareType::BBishop => html! { <StyledSVG class={ black } svg={ BISHOP_SVG }/> },
        SquareType::BQueen  => html! { <StyledSVG class={ black } svg={ QUEEN_SVG }/> },
        SquareType::BPawn   => html! { <StyledSVG class={ black } svg={ PAWN_SVG }/> },
        SquareType::BKing   => html! { <StyledSVG class={ black } svg={ KING_SVG }/> },
        SquareType::Empty   => html! {},
    }
}
