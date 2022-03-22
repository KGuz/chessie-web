use crate::components::{square::*, styled_svg::*};
use crate::resources::*;
use yew::{html, virtual_dom::vlist::VList, Callback, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct BoardProps {
    pub callback: Callback<SquareData>,
    pub chessboard: [SquareType; 64],
}

pub struct Board;
impl Component for Board {
    type Message = ();
    type Properties = BoardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = &ctx.props().callback;
        let board = &ctx.props().chessboard;

        let mut square_components = VList::new();
        square_components.add_children((0..64).rev().map(|n| {
            html! { <Square { callback } idx={ n } square={ board[n] }/> }
        }));

        html! {
            <>
                <StyledSVG class="chessboard-svg" svg={ CHESSBOARD_SVG }/>
                <div class="board">{ square_components }</div>
            </>
        }
    }
}
