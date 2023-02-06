use super::styled_svg::*;
use crate::resources::*;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{DragEvent, MouseEvent};
use yew::{html, Component, Context, Html, virtual_dom::VNode};

pub enum DragAndDropMsg {
    DisplayMedia(VNode),
    RemoveFile,
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
            // let freader = web_sys::FileReader::new().unwrap_throw();
            // freader.read_as_array_buffer(&file).unwrap_throw();

            let img = html!{
                <img src={url} class="media-file"/>
            };
            DragAndDropMsg::DisplayMedia(img)
        });

        let onclick = ctx.link().callback(|_event: MouseEvent| {
            DragAndDropMsg::RemoveFile
        });

        let ondragover = |event: DragEvent| {
            event.prevent_default();
            event.stop_propagation();
        };
        
        if let Some(file) = self.file.clone() {
            html! {
                <div class="media-dragNdrop-zone">
                    <input class="dragNdrop-input" {ondrop} {ondragover}/>
                    <div class="discard-media-icon" {onclick}>
                        <StyledSVG class="discard-media-svg" svg={ XMARK_SVG }/>
                    </div>
                    {file}
                </div>
            }
        } else {
            html! {
                <div class="dragNdrop-zone">
                    <input class="dragNdrop-input" {ondrop} {ondragover}/>
                    <div class="dragNdrop-info ">
                        <div class="dragNdrop-icon"><StyledSVG class="upload-svg" svg={ UPLOAD_SVG }/></div>
                        <div class="dragNdrop-text-title">{ "Upload position" }</div>
                        <div class="dragNdrop-text-desc">
                            { "Drag and drop files here to upload. " }
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
            DragAndDropMsg::RemoveFile => {
                self.file = None;
                true
            }
        }
    }
}
