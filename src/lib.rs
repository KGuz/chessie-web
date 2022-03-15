pub mod chessboard {
    use yew::prelude::*;

    enum Msg {
        PreviewChange,
        DetailsChange,
        EvaluationStatus,
    }
    
    struct MainComponent {
        frame_tab: bool,
        transcript_tab: bool,
        evaluation: isize,
    }
    
    impl Component for MainComponent {
        type Message = Msg;
        type Properties = ();
    
        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                frame_tab: true,
                transcript_tab: true,
                evaluation: 0,
            }
        }
    
        fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Msg::DetailsChange => true,
                Msg::PreviewChange => true,
                Msg::EvaluationStatus => true,
            }
        }
    
        fn view(&self, ctx: &Context<Self>) -> Html {
            let _link = ctx.link();
            html! {
                <div class="main">
                    <div class="container" style="border: 4px solid var(--purple); background-color: rgba(165, 75, 115, 0.1);">
                        <p>{ "OTB Chess Inference Engine + ChessBoard" }</p>
                    </div>
                    <div class="container" style="border: 4px solid var(--cyan); background-color: rgba(87, 142, 87, 0.1);">
                        <p>{ "Frame preview + Trnascript" }</p>
                    </div>
                </div>
            }
        }
    }    
}