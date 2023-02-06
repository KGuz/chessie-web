use crate::components::{square::*, styled_svg::*};
use crate::resources::*;
use yew::{html, virtual_dom::vlist::VList, Callback, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct FenFieldProps {
    pub chessboard: [SquareType; 64],
}

pub struct FenField;
impl Component for FenField {
    type Message = ();
    type Properties = FenFieldProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="fen-field">
                <textarea class="fen-field-textarea" placeholder="r1bQkb1r/1q1npp1p/3pN1pP/6P1/N1p1PP2/4B3/PPP5/2KR3R b - - 6 25" readonly={true}>
                    { format!("{}", board_to_fen(ctx.props().chessboard)) }
                </textarea>
                <StyledSVG class="copy-svg" svg = { CLIPBOARD_SVG }/>
            </div>
        }
    }
}

fn board_to_fen(_chessboard: &[SquareType; 64]) -> String {
    String::from("r1bQkb1r/1q1npp1p/3pN1pP/6P1/N1p1PP2/4B3/PPP5/2KR3R b - - 6 25")
}