use std::fmt::Result;
use std::thread;

use crate::data_parser::playback_record::PlaybackRecord;
use crate::data_parser::processing::filter_by;
use crate::Route;

use super::components::data_context::DataContext;
use super::components::file_upload::FileInput;
use chrono::format;
use chrono::prelude::*;
use gloo::console::log;
use gloo::file::FileReadError;
use rayon::iter::IntoParallelIterator;
use tokio::task;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::navigator;

#[derive(PartialEq, Debug)]
pub enum FileState {
    NotUploaded,
    Uploading,
    Processing,
    Processed,
}

#[function_component]
pub fn FileUploadView() -> Html {
    let file_contents: UseStateHandle<Vec<Vec<u8>>> = use_state(|| Vec::<Vec<u8>>::new());
    let file_state = use_state(|| FileState::NotUploaded);
    let file_ammount = use_state(|| 0);
    let data_context = use_context::<DataContext>().unwrap();
    let navigator = use_navigator().unwrap();
    use_effect_with(file_state.clone(), {
        let file_state = file_state.clone();
        let data_context = data_context.clone();
        let file_contents = (*file_contents).clone();
        let navigator = navigator.clone();
        move |_| {
            if *file_state != FileState::Processing {
                return;
            }

            let parsed_data = PlaybackRecord::from_jsons(&file_contents);

            match parsed_data {
                Ok(data) => {
                    data_context.dispatch(filter_by(&data, |record| {
                        record.ts.date_naive()
                            >= Utc::now()
                                .with_year(Utc::now().year() - 1)
                                .unwrap()
                                .date_naive()
                    }));
                    file_state.set(FileState::Processed);
                }
                Err(e) => navigator.push(&Route::DataError),
            }
        }
    });
    let on_file_ammount = {
        let file_ammount = file_ammount.clone();
        Callback::from(move |n| {
            file_ammount.set(n);
            log!(format!("New file ammount set: {}", n))
        })
    };
    let on_finish_loading = {
        let loading = file_state.clone();
        Callback::from(move |state: FileState| {
            loading.set(state);
        })
    };

    let on_button_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::StatsHome))
    };
    let on_state_change = {
        let file_contents = file_contents.clone();
        let file_ammount = file_ammount.clone();
        let file_state = file_state.clone();
        Callback::from(move |records: Vec<u8>| {
            let mut new_contents = (*file_contents).clone();
            new_contents.push(records);
            let current_ammount = new_contents.len();

            file_contents.set(new_contents);

            if *file_ammount <= current_ammount {
                file_state.set(FileState::Processing);
            }
        })
    };
    html! {
        <div class="grid grid-rows-[50%_50%] h-full w-full">
        <div class="flex flex-col items-center justify-center">
          <img src="assets/logo.svg" alt="logo" class="w-full max-h-[30vh]" />
          <h1 class="text-8xl text-center font-bold my-8">{"ReWrapped"}</h1>
        </div>

        <div class="flex flex-col items-center justify-center">
            if *file_state == FileState::Processed {
               <button onclick={on_button_click} class="mbtn text-4xl font-semibold  py-3 h-16"> {"Get ReWrapped!"}</button>
            } else if *file_state == FileState::Processing {
                <div class="flex items-center justify-center w-1/2 text-xl font-semibold py-3 mbtn">{"Proccessing..."}</div>
                <p class="text-base text-gray-400 mt-8 hover:underline text-center">
                        { "This may take a while..." }
                </p>
            } else {<FileInput on_file_loaded={on_state_change} on_change_upload_state={on_finish_loading} on_file_ammount= {on_file_ammount}/>
                <p class="text-base text-gray-400 mt-8 hover:underline text-center">
                    <a href="/upload-guide" >
                        { "What do I need to upload?" }
                    </a>
                </p>
            }
        </div>

    </div>
              }
}
