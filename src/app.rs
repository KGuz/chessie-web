use crate::{
    components::{square::*, styled_svg::*},
    resources::*,
};
use yew::{html, virtual_dom::vlist::VList, Component, Context, Html};

#[derive(Clone, Copy)]
pub enum Msg {
    UpdateBoard(SquareData),
}

pub struct App {
    chessboard: [SquareType; 64],
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = &ctx.link().callback(move |sq| Msg::UpdateBoard(sq));

        let mut square_components = VList::new();
        square_components.add_children((0..64).rev().map(|n| {
            html! { <SquareComponent { callback } idx={ n } square={ self.chessboard[n] }/> }
        }));

        html! {
            <main>
                <header><a>{ "Chess Inference Engine" }</a></header>
                <div class="chess">
                    <StyledSVGComponent class="chessboard-svg" svg={ CHESSBOARD_SVG }/>
                    <div class="board">{ square_components }</div>
                    <div class="evaluation"></div>
                </div>
                <div class="preview"></div>
                <div class="details"></div>
            </main>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateBoard(SquareData { pos, typ }) => {
                self.chessboard[pos] = typ;
                false
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            chessboard: [
                SquareType::WRook,
                SquareType::WKnight,
                SquareType::WBishop,
                SquareType::WQueen,
                SquareType::WKing,
                SquareType::WBishop,
                SquareType::WKnight,
                SquareType::WRook,
                SquareType::WPawn,
                SquareType::WPawn,
                SquareType::WPawn,
                SquareType::WPawn,
                SquareType::WPawn,
                SquareType::WPawn,
                SquareType::WPawn,
                SquareType::WPawn,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::Empty,
                SquareType::BPawn,
                SquareType::BPawn,
                SquareType::BPawn,
                SquareType::BPawn,
                SquareType::BPawn,
                SquareType::BPawn,
                SquareType::BPawn,
                SquareType::BPawn,
                SquareType::BRook,
                SquareType::BKnight,
                SquareType::BBishop,
                SquareType::BQueen,
                SquareType::BKing,
                SquareType::BBishop,
                SquareType::BKnight,
                SquareType::BRook,
            ],
        }
    }
}
