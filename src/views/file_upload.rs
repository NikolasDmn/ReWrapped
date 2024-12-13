use crate::data_parser::playback_record::PlaybackRecord;
use crate::Route;

use super::components::data_context::DataContext;
use super::components::file_upload::FileInput;
use gloo::console::log;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::navigator;

#[derive(PartialEq)]
pub enum FileState {
    NotUploaded,
    Uploading,
    Processed,
}
#[function_component]
pub fn FileUploadView() -> Html {
    let file_contents = use_state(|| vec![]);
    let file_state = use_state(|| FileState::NotUploaded);
    let file_ammount = use_state(|| 0);
    let data_context = use_context::<DataContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let on_state_change = {
        let file_contents = file_contents.clone();
        let file_ammount = file_ammount.clone();
        let file_state = file_state.clone();
        let navigator = navigator.clone();
        Callback::from(
            move |records: Result<Vec<PlaybackRecord>, serde_json::Error>| {
                if records.is_err() {
                    navigator.push(&Route::DataError);
                    return;
                }
                let mut new_contents = (*file_contents).clone();
                new_contents.push(records.unwrap());
                let current_ammount = new_contents.len();
                if *file_ammount == current_ammount {
                    data_context.dispatch(new_contents.clone().into_iter().flatten().collect());
                    file_state.set(FileState::Processed);
                } else {
                    file_contents.set(new_contents);
                }
            },
        )
    };
    let on_file_ammount = {
        let file_ammount = file_ammount.clone();
        Callback::from(move |n| file_ammount.set(n))
    };
    let on_finish_loading = {
        let loading = file_state.clone();
        Callback::from(move |state: FileState| {
            loading.set(state);
        })
    };

    let on_button_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::TopArtists))
    };
    html! {
        <div class="grid grid-rows-[50%_50%] h-full w-full">
        <div class="flex flex-col items-center justify-center">
          <img src="assets/logo/logo.svg" alt="logo" class="w-full max-h-[30vh]" />
          <h1 class="text-8xl text-center font-bold my-8">{"ReWrapped"}</h1>
        </div>

        <div class="flex flex-col items-center justify-center">
          <div class={ if *file_state != FileState::NotUploaded {"invisible w-full"} else {"w-full"}}>
                 <FileInput on_file_loaded={on_state_change} on_change_upload_state={on_finish_loading} on_file_ammount= {on_file_ammount}/>
          <p class="text-base text-gray-400 mt-8 hover:underline text-center">
            <a href="/upload-guide" >
              { "What do I need to upload?" }
            </a>
          </p>

            </div>
            if *file_state == FileState::Uploading {
                <div class="flex items-center justify-center w-1/2 bg-primary text-xl font-semibold text-white py-3 rounded-lg"><span class="loading loading-dots loading-lg"></span></div>
            }
            else if *file_state == FileState::Processed {
                <button onclick={on_button_click} class="btn btn-primary text-4xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"> {"Get ReWrapped!"}</button>
                }
        </div>

    </div>
              }
}
