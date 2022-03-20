use crate::components::{square::*, styled_svg::*};
use crate::resources::*;
use yew::{html, virtual_dom::vlist::VList, Callback, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct BoardComponentProps {
    pub callback: Callback<SquareData>,
    pub chessboard: [SquareType; 64],
}

pub struct BoardComponent;
impl Component for BoardComponent {
    type Message = ();
    type Properties = BoardComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = &ctx.props().callback;
        let board = &ctx.props().chessboard;

        let mut square_components = VList::new();
        square_components.add_children((0..64).rev().map(|n| {
            html! { <SquareComponent { callback } idx={ n } square={ board[n] }/> }
        }));

        html! {
            <>
                <StyledSVGComponent class="chessboard-svg" svg={ CHESSBOARD_SVG }/>
                <div class="board">{ square_components }</div>
            </>
        }
    }
}
