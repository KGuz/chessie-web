use crate::components::{board::*, evalbar::*, square::*, dragndrop::*, fenfield::*};
use yew::{html, Component, Context, Html};

#[derive(Clone, Copy)]
pub enum Msg {
    UpdateBoard(SquareData),
    PropagateBoardState([SquareType; 64]),
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

        html! {
            <main>
                <header><a>{ "Chess Inference Engine" }</a></header>
                <div class="chess">
                    <Board {callback} chessboard={ self.chessboard }/>
                    <EvalBarComponent ..Evaluation::Value(0.0).into()/>
                </div>
                <div class="preview">
                    <DragAndDrop />
                </div>
                <div class="details">
                    <FenField chessboard={ self.chessboard }/>
                </div>
            </main>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateBoard(SquareData { pos, typ }) => {
                self.chessboard[pos] = typ;
                true
            }
            Msg::PropagateBoardState(_state) => false,
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
