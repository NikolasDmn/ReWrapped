use crate::data_parser::playback_record::PlaybackRecord;

use super::super::file_upload::FileState;
use _FileInputProps::{on_file_ammount, on_file_loaded};
use gloo::console::log;
use gloo::file::callbacks::{read_as_text, FileReader};
use gloo::file::{File, FileReadError};
use gloo::utils::errors::JsError;
use js_sys::Promise;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures;
use web_sys::{DragEvent, Event, HtmlInputElement};
use yew::prelude::*;
use yew_router::navigator;

#[derive(Properties, PartialEq)]
pub struct FileInputProps {
    pub on_file_loaded: Callback<Vec<u8>>,
    pub on_change_upload_state: Callback<FileState>,
    pub on_file_ammount: Callback<usize>,
}
pub struct FileDetails {
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(FileDetails),
    Files(Option<web_sys::FileList>),
}
pub struct FileInput {
    readers: HashMap<String, FileReader>,
    uploading: bool,
}
async fn read_file_as_text(file: File) -> Result<String, JsValue> {
    let (tx, rx) = std::sync::mpsc::channel();

    let _reader = read_as_text(&file, move |res| {
        tx.send(res).unwrap_or_else(|_| {
            log!("Failed to send file contents via channel.");
        });
    });

    // Block until we receive the file read result from the callback
    match rx.recv() {
        Ok(Ok(content)) => Ok(content),
        Ok(Err(err)) => Err(JsValue::from_str(&err.to_string())),
        Err(_) => Err(JsValue::from_str("Failed to receive file read result")),
    }
}
impl From<Result<Vec<u8>, FileReadError>> for Msg {
    fn from(value: Result<Vec<u8>, FileReadError>) -> Msg {
        match value {
            Ok(s) => Msg::Loaded(FileDetails { data: s }),
            Err(_) => Msg::Loaded(FileDetails { data: vec![] }),
        }
    }
}
impl Component for FileInput {
    type Message = Msg;
    type Properties = FileInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            uploading: false,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                ctx.props().on_file_loaded.emit(file.data);
                log!("Sending file...");
                true
            }
            Msg::Files(files) => {
                let files = gloo::file::FileList::from(files.expect("files"));
                ctx.props().on_file_ammount.emit(files.len());
                self.uploading = true;
                for file in files.iter() {
                    ctx.props()
                        .on_change_upload_state
                        .emit(FileState::Uploading);

                    let name = file.name().clone();
                    gloo::console::log!(format!("Registering file: {}", name).as_str());
                    ctx.link()
                        .send_future(gloo::file::futures::read_as_bytes(file));
                }
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {

                           <div class="flex items-center justify-center w-full">
            if !self.uploading{
              <label
                for="file-upload"
                class="flex items-center justify-center w-1/2 bg-primary text-xl font-semibold text-white py-3 cursor-pointer rounded-lg"
              >
                        {"Select Files"}
            </label>

              <input
                id="file-upload"
                type="file"
                accept="image/*,video/*"
                class="hidden"
                multiple={true}
                accept=".json"
                onchange={ctx.link().callback(move |e: Event| {
                  let input: HtmlInputElement = e.target_unchecked_into();
                  Msg::Files(input.files())
                })}
              />}
            else {
                <div class ="flex flex-col items-center justify-center w-full h-full ">
                    <div class="flex items-center justify-center w-1/2 bg-primary text-xl font-semibold text-white py-3 rounded-lg">{"Uploading... "}</div>
                    <p class="text-base text-gray-400 mt-8 hover:underline text-center">
                        { "This may take a while..." }
                    </p>
                </div>
            }
            </div>

        }
    }
}
