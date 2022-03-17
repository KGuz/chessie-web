use yew::{function_component, html};
use yew_chess::resources::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="main">
            <header><a>{ "Chess Inference Engine" }</a></header>
            <div class="chess">
                <StyledSVG class="chessboard-svg" svg={ CHESSBOARD_SVG }/>
                <div class="board">
                    <button class="square" id="h8"><StyledSVG class="black-pice-svg" svg={ ROOK_SVG }/></button>
                    <button class="square" id="g7"><StyledSVG class="black-pice-svg" svg={ PAWN_SVG }/></button>
                    <button class="square" id="f6"><StyledSVG class="white-pice-svg" svg={ KING_SVG }/></button>
                    <button class="square" id="e5"><StyledSVG class="black-pice-svg" svg={ QUEEN_SVG }/></button>
                    <button class="square" id="d4"><StyledSVG class="white-pice-svg" svg={ BISHOP_SVG }/></button>
                    <button class="square" id="c3"><StyledSVG class="black-pice-svg" svg={ KNIGHT_SVG }/></button>
                    <button class="square" id="b2"><StyledSVG class="white-pice-svg" svg={ PAWN_SVG }/></button>
                    <button class="square" id="a1"><StyledSVG class="white-pice-svg" svg={ ROOK_SVG }/></button>
                </div>
                <div class="evaluation"></div>
            </div>
            <div class="preview"></div>
            <div class="details"></div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}

// <button class="square" id="a8"></button>
// <button class="square" id="b8"></button>
// <button class="square" id="c8"></button>
// <button class="square" id="d8"></button>
// <button class="square" id="e8"></button>
// <button class="square" id="f8"></button>
// <button class="square" id="g8"></button>
// <button class="square" id="h8"></button>
// <button class="square" id="a7"></button>
// <button class="square" id="b7"></button>
// <button class="square" id="c7"></button>
// <button class="square" id="d7"></button>
// <button class="square" id="e7"></button>
// <button class="square" id="f7"></button>
// <button class="square" id="g7"></button>
// <button class="square" id="h7"></button>
// <button class="square" id="a6"></button>
// <button class="square" id="b6"></button>
// <button class="square" id="c6"></button>
// <button class="square" id="d6"></button>
// <button class="square" id="e6"></button>
// <button class="square" id="f6"></button>
// <button class="square" id="g6"></button>
// <button class="square" id="h6"></button>
// <button class="square" id="a5"></button>
// <button class="square" id="b5"></button>
// <button class="square" id="c5"></button>
// <button class="square" id="d5"></button>
// <button class="square" id="e5"></button>
// <button class="square" id="f5"></button>
// <button class="square" id="g5"></button>
// <button class="square" id="h5"></button>
// <button class="square" id="a4"></button>
// <button class="square" id="b4"></button>
// <button class="square" id="c4"></button>
// <button class="square" id="d4"></button>
// <button class="square" id="e4"></button>
// <button class="square" id="f4"></button>
// <button class="square" id="g4"></button>
// <button class="square" id="h4"></button>
// <button class="square" id="a3"></button>
// <button class="square" id="b3"></button>
// <button class="square" id="c3"></button>
// <button class="square" id="d3"></button>
// <button class="square" id="e3"></button>
// <button class="square" id="f3"></button>
// <button class="square" id="g3"></button>
// <button class="square" id="h8"></button>
// <button class="square" id="a2"></button>
// <button class="square" id="b2"></button>
// <button class="square" id="c2"></button>
// <button class="square" id="d2"></button>
// <button class="square" id="e2"></button>
// <button class="square" id="f2"></button>
// <button class="square" id="g2"></button>
// <button class="square" id="h2"></button>
// <button class="square" id="a1"></button>
// <button class="square" id="b1"></button>
// <button class="square" id="c1"></button>
// <button class="square" id="d1"></button>
// <button class="square" id="e1"></button>
// <button class="square" id="f1"></button>
// <button class="square" id="g1"></button>
// <button class="square" id="h1"></button>