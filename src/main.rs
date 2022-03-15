use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="main">
            <div class="left-pane">
                <div class="upper-left-pane" 
                // style="border: 2px solid var(--yellow); background-color: rgba(204, 136, 16, 0.1);"
                >
                    <div class="header"
                    // style="border: 2px solid var(--purple); background-color: rgba(65, 75, 115, 0.1);" 
                    >{ "OTB Chess Inference Engine" }</div>
                </div>
                <div class="lower-left-pane">
                    <div class="chessboard-pane">
                        <div class="numbers"
                        // style="border: 2px solid var(--purple); background-color: rgba(165, 75, 115, 0.1);"
                        >
                            <div>{ "8" }</div>
                            <div>{ "7" }</div>
                            <div>{ "6" }</div>
                            <div>{ "5" }</div>
                            <div>{ "4" }</div>
                            <div>{ "3" }</div>
                            <div>{ "2" }</div>
                            <div>{ "1" }</div>
                        </div>
                        <div class="board" 
                        // style="border: 2px solid var(--blue); background-color: rgba(55, 115, 117, 0.1);"
                        >  
                            <button id="8a">{ "8a" }</button>
                            <button id="8b">{ "8b" }</button>
                            <button id="8c">{ "8c" }</button>
                            <button id="8d">{ "8d" }</button>
                            <button id="8e">{ "8e" }</button>
                            <button id="8f">{ "8f" }</button>
                            <button id="8g">{ "8g" }</button>
                            <button id="8h">{ "8h" }</button>
                            <button id="7a">{ "7a" }</button>
                            <button id="7b">{ "7b" }</button>
                            <button id="7c">{ "7c" }</button>
                            <button id="7d">{ "7d" }</button>
                            <button id="7e">{ "7e" }</button>
                            <button id="7f">{ "7f" }</button>
                            <button id="7g">{ "7g" }</button>
                            <button id="7h">{ "7h" }</button>
                            <button id="6a">{ "6a" }</button>
                            <button id="6b">{ "6b" }</button>
                            <button id="6c">{ "6c" }</button>
                            <button id="6d">{ "6d" }</button>
                            <button id="6e">{ "6e" }</button>
                            <button id="6f">{ "6f" }</button>
                            <button id="6g">{ "6g" }</button>
                            <button id="6h">{ "6h" }</button>
                            <button id="5a">{ "5a" }</button>
                            <button id="5b">{ "5b" }</button>
                            <button id="5c">{ "5c" }</button>
                            <button id="5d">{ "5d" }</button>
                            <button id="5e">{ "5e" }</button>
                            <button id="5f">{ "5f" }</button>
                            <button id="5g">{ "5g" }</button>
                            <button id="5h">{ "5h" }</button>
                            <button id="4a">{ "4a" }</button>
                            <button id="4b">{ "4b" }</button>
                            <button id="4c">{ "4c" }</button>
                            <button id="4d">{ "4d" }</button>
                            <button id="4e">{ "4e" }</button>
                            <button id="4f">{ "4f" }</button>
                            <button id="4g">{ "4g" }</button>
                            <button id="4h">{ "4h" }</button>
                            <button id="3a">{ "3a" }</button>
                            <button id="3b">{ "3b" }</button>
                            <button id="3c">{ "3c" }</button>
                            <button id="3d">{ "3d" }</button>
                            <button id="3e">{ "3e" }</button>
                            <button id="3f">{ "3f" }</button>
                            <button id="3g">{ "3g" }</button>
                            <button id="8h">{ "8h" }</button>
                            <button id="2a">{ "2a" }</button>
                            <button id="2b">{ "2b" }</button>
                            <button id="2c">{ "2c" }</button>
                            <button id="2d">{ "2d" }</button>
                            <button id="2e">{ "2e" }</button>
                            <button id="2f">{ "2f" }</button>
                            <button id="2g">{ "2g" }</button>
                            <button id="2h">{ "2h" }</button>
                            <button id="1a">{ "1a" }</button>
                            <button id="1b">{ "1b" }</button>
                            <button id="1c">{ "1c" }</button>
                            <button id="1d">{ "1d" }</button>
                            <button id="1e">{ "1e" }</button>
                            <button id="1f">{ "1f" }</button>
                            <button id="1g">{ "1g" }</button>
                            <button id="1h">{ "1h" }</button>
                        </div>
                        <div class="letters"
                        // style="border: 2px solid var(--yellow); background-color: rgba(204, 136, 16, 0.1);"
                        >
                            <div>{ "A" }</div>
                            <div>{ "B" }</div>
                            <div>{ "C" }</div>
                            <div>{ "D" }</div>
                            <div>{ "E" }</div>
                            <div>{ "F" }</div>
                            <div>{ "G" }</div>
                            <div>{ "H" }</div>
                        </div>
                    </div>
                    <div class="evaluation"
                    // style="border: 2px solid var(--purple); background-color: rgba(165, 75, 115, 0.1);"
                    >
                        <div class="evaluation-bar"
                        // style="border: 2px solid var(--purple); background-color: rgba(165, 75, 115, 0.1);"
                        >
                            <div class="evaluation-bar-fill"
                            // style="border: 2px solid var(--yellow); background-color: rgba(204, 136, 16, 0.1);"
                            ></div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="right-pane" style="">
                <div class="upper-right-pane" 
                style="border: 2px solid var(--blue); background-color: rgba(55, 115, 117, 0.1);" 
                ></div>    
                <div class="lower-right-pane" 
                style="border: 2px solid var(--purple); background-color: rgba(165, 75, 115, 0.1);"
                ></div>    
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
