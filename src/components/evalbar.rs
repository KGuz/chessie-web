use std::cmp::Ordering::*;
use yew::{function_component, html, Properties};

#[derive(Clone, Copy, PartialEq)]
pub enum Evaluation {
    Value(f32),
    MateIn(i8),
    Result(i8),
    Impossible,
}

#[derive(Properties, PartialEq)]
pub struct EvalBarProps {
    pub position: String,
    pub value: String,
    pub fill: usize,
}

impl From<Evaluation> for EvalBarProps {
    fn from(evaluation: Evaluation) -> Self {
        match evaluation {
            Evaluation::Value(val) => {
                let l = &format!("{:.*}", if val.abs() >= 10.0 { 0 } else { 1 }, val.abs()) as &str;
                let f = |mut x: f32| {
                    x = x.clamp(-6.9, 6.9);
                    (0.069 * x.powi(3) - 10.0 * x + 50.0) as usize
                };

                EvalBarProps::from(match val.partial_cmp(&0.0).unwrap_or(Equal) {
                    Less => ("flex-start", l, f(val)),
                    Equal => ("center", l, 50),
                    Greater => ("flex-end", l, f(val)),
                })
            }
            Evaluation::MateIn(num) => {
                let m = &format!("M{}", num.abs()) as &str;

                EvalBarProps::from(match num.cmp(&0) {
                    Less => ("flex-start", m, 100),
                    Equal => ("center", m, 50),
                    Greater => ("flex-end", m, 0),
                })
            }
            Evaluation::Result(win) => EvalBarProps::from(match win.cmp(&0) {
                Less => ("flex-start", "0-1", 100),
                Equal => ("center", "½-½", 50),
                Greater => ("flex-end", "1-0", 0),
            }),
            Evaluation::Impossible => ("center", "??", 50).into(),
        }
    }
}

impl From<(&str, &str, usize)> for EvalBarProps {
    fn from((p, v, f): (&str, &str, usize)) -> Self {
        Self { position: p.into(), value: v.into(), fill: f }
    }
}

#[function_component(EvalBarComponent)]
pub fn eval_bar_component(EvalBarProps { position, value, fill }: &EvalBarProps) -> Html {
    html! {
        <div class="evaluation">
            <div class="bar-label" style={ format!("align-self: {}", position) }>{ value }</div>
            <div class="bar-outline">
                <div class="bar-fill" style={ format!("height: {}%", fill) }/>
            </div>
        </div>
    }
}
