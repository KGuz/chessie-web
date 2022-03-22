use super::styled_svg::*;
use crate::resources::*;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{DragEvent};
use yew::{html, Component, Context, Html, virtual_dom::VNode};

pub enum DragAndDropMsg {
    DisplayMedia(VNode)
}

pub struct DragAndDrop {
    file: Option<VNode>,
}
impl Component for DragAndDrop {
    type Message = DragAndDropMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            file: None
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ondrop = ctx.link().callback(|event: DragEvent| {
            event.prevent_default();

            let data = event.data_transfer().unwrap_throw();
            let file = data.files().unwrap_throw().item(0).unwrap_throw();
            
            let url = web_sys::Url::create_object_url_with_blob(&file).unwrap_throw();
            let img = html!{
                <img src={url} style="width: 35vw; height: auto; align-self: center;"/>
            };
            DragAndDropMsg::DisplayMedia(img)
        });

        let ondragover = |event: DragEvent| {
            event.prevent_default();
        };
        
        if let Some(file) = self.file.clone() {
            html! {
                {file}
            }
        } else {
            html! {
                <div class="dragNdrop-zone" {ondrop} {ondragover}>
                    <div class="dragNdrop-info">
                        <div class="dragNdrop-icon"><StyledSVG class="upload-svg" svg={ UPLOAD_SVG }/></div>
                        <div class="dragNdrop-text-title">{ "Upload position" }</div>
                        <div class="dragNdrop-text-desc">
                            { "Drag and drop files here to upload." }<br/>
                            { "Only files that are less than 500mb in size will be accepted" }
                        </div>
                    </div>
                </div>
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DragAndDropMsg::DisplayMedia(file) => {
                self.file = Some(file);
                true
            }
        }
    }
}
