use crate::data_parser::playback_record::PlaybackRecord;

use super::super::file_upload::FileState;
use _FileInputProps::on_file_ammount;
use gloo::file::callbacks::FileReader;
use std::collections::HashMap;
use web_sys::{DragEvent, Event, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileInputProps {
    pub on_file_loaded: Callback<Result<Vec<PlaybackRecord>, serde_json::Error>>,
    pub on_change_upload_state: Callback<FileState>,
    pub on_file_ammount: Callback<usize>,
}
pub struct FileDetails {
    name: String,
    file_type: String,
    data: String,
}

pub enum Msg {
    Loaded(FileDetails),
    Files(Option<web_sys::FileList>),
}
pub struct FileInput {
    readers: HashMap<String, FileReader>,
}
impl Component for FileInput {
    type Message = Msg;
    type Properties = FileInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.readers.remove(&file.name);
                ctx.props()
                    .on_file_loaded
                    .emit(PlaybackRecord::from_json(&file.data));
                true
            }
            Msg::Files(files) => {
                let files = gloo::file::FileList::from(files.expect("files"));
                ctx.props().on_file_ammount.emit(files.len());
                for file in files.iter() {
                    ctx.props()
                        .on_change_upload_state
                        .emit(FileState::Uploading);
                    let link = ctx.link().clone();
                    let name = file.name().clone();
                    let file_type = file.raw_mime_type();
                    gloo::console::log!(format!("Registering file: {}", name).as_str());
                    let task = {
                        gloo::file::callbacks::read_as_text(file, move |res| {
                            link.send_message(Msg::Loaded(FileDetails {
                                data: res.expect("failed to read file"),
                                file_type,
                                name,
                            }))
                        })
                    };
                    self.readers.insert(file.name(), task);
                }
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
                       <div class="flex items-center justify-center w-full">
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
          />
        </div>
                }
    }
}
