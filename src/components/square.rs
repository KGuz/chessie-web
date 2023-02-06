use crate::components::piece::*;
use web_sys::KeyboardEvent;
use yew::{html, Callback, Component, Context, Html, Properties};

pub enum SuqareMsg {
    Replace(KeyboardEvent),
}

#[derive(PartialEq, Properties)]
pub struct SquareProps {
    pub callback: Callback<SquareData>,
    pub idx: usize,
    pub square: SquareType,
}

pub struct Square {
    square: SquareData,
    notation: String,
}

impl Component for Square {
    type Message = SuqareMsg;
    type Properties = SquareProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            square: SquareData { pos: ctx.props().idx, typ: ctx.props().square },
            notation: algebraic_noatation(ctx.props().idx),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeydown = ctx.link().callback(|e: KeyboardEvent| {
            SuqareMsg::Replace(e)
        });

        html! {
            <button {onkeydown} class="square" id={ self.notation.clone() }>
                <PieceComponent typ={ self.square.typ }/>
            </button>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SuqareMsg::Replace(e) => match e.key().as_str() {
                "Delete" | "Backspace" => {
                    self.square.typ = SquareType::Empty;
                    ctx.props().callback.emit(self.square);
                    true
                }
                key if SquareType::from(key) != SquareType::Empty => {
                    let typ = SquareType::from(key);
                    if typ != self.square.typ {
                        self.square.typ = typ;
                        ctx.props().callback.emit(self.square);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SquareData {
    pub pos: usize,
    pub typ: SquareType,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SquareType {
    WPawn,
    WRook,
    WKnight,
    WBishop,
    WQueen,
    WKing,
    BPawn,
    BRook,
    BKnight,
    BBishop,
    BQueen,
    BKing,
    Empty,
}

impl From<&str> for SquareType {
    fn from(id: &str) -> Self {
        if id.len() == 1 {
            match id {
                "r" => SquareType::WRook,
                "n" => SquareType::WKnight,
                "b" => SquareType::WBishop,
                "q" => SquareType::WQueen,
                "k" => SquareType::WKing,
                "p" => SquareType::WPawn,
                "R" => SquareType::BRook,
                "N" => SquareType::BKnight,
                "B" => SquareType::BBishop,
                "Q" => SquareType::BQueen,
                "K" => SquareType::BKing,
                "P" => SquareType::BPawn,
                _ => SquareType::Empty,
            }
        } else {
            match id.to_lowercase().as_str() {
                "wr" | "rw" => SquareType::WRook,
                "wn" | "nw" => SquareType::WKnight,
                "wb" | "bw" => SquareType::WBishop,
                "wq" | "qw" => SquareType::WQueen,
                "wk" | "kw" => SquareType::WKing,
                "wp" | "pw" => SquareType::WPawn,
                "br" | "rb" => SquareType::BRook,
                "bn" | "nb" => SquareType::BKnight,
                "bb" => SquareType::BBishop,
                "bq" | "qb" => SquareType::BQueen,
                "bk" | "kb" => SquareType::BKing,
                "bp" | "pb" => SquareType::BPawn,
                _ => SquareType::Empty,
            }
        }
    }
}

impl From<String> for SquareType {
    fn from(id: String) -> Self {
        Self::from(id.as_str())
    }
}

pub fn algebraic_noatation(idx: usize) -> String {
    let l = idx % 8;
    let n = (idx - l) / 8;

    let l = 7 - l as u8 + b'a';
    let n = n as u8 + b'1';

    format!("{}{}", l as char, n as char)
}
