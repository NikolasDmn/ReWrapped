use chart::ChartType;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};

use crate::Route;

pub mod chart;

#[function_component(StatsHome)]
pub fn stats_home() -> Html {
    let navigator = use_navigator().unwrap();
    html! {
           <div><h2>{"Charts"}</h2>
       <div class="grid grid-cols-2 gap-4">
    <button
           class="btn btn-primary text-2xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"
           onclick={Callback::from({
               let navigator = navigator.clone();
               move |_| {
                   navigator.push(&Route::StatCharts { chart_type: ChartType::Artists });
               }
           })}
       >
           {"Artists"}
       </button>
       <button
           class="btn btn-primary text-2xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"
           onclick={Callback::from({
               let navigator = navigator.clone();
               move |_| {
                   navigator.push(&Route::StatCharts { chart_type: ChartType::Songs });
               }
           })}
       >
           {"Songs"}
       </button>
       <button
           class="btn btn-primary text-2xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"
           onclick={Callback::from({
               let navigator = navigator.clone();
               move |_| {
                   navigator.push(&Route::StatCharts { chart_type: ChartType::Albums });
               }
           })}
       >
           {"Albums"}
       </button>
       <button
           class="btn btn-primary text-2xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"
           onclick={Callback::from({
               let navigator = navigator.clone();
               move |_| {
                   navigator.push(&Route::StatCharts { chart_type: ChartType::Week });
               }
           })}
       >
           {"Week"}
       </button>
       <button
           class="btn btn-primary text-2xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"
           onclick={Callback::from({
               let navigator = navigator.clone();
               move |_| {
                   navigator.push(&Route::StatCharts { chart_type: ChartType::Months });
               }
           })}
       >
           {"Months"}
       </button>
       <button
           class="btn btn-primary text-2xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"
           onclick={Callback::from({
               let navigator = navigator.clone();
               move |_| {
                   navigator.push(&Route::StatCharts { chart_type: ChartType::Platform });
               }
           })}
       >
           {"Platform"}
       </button>
       <button
           class="btn btn-primary text-2xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"
           onclick={Callback::from({
               let navigator = navigator.clone();
               move |_| {
                   navigator.push(&Route::StatCharts { chart_type: ChartType::Country });
               }
           })}
       >
           {"Country"}
       </button>
        <button
           class="btn btn-primary text-2xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"
           onclick={Callback::from({
               let navigator = navigator.clone();
               move |_| {
                   navigator.push(&Route::StatCharts { chart_type: ChartType::Days });
               }
           })}
       >
           {"Days"}
       </button>
    <button
           class="btn btn-primary text-2xl font-semibold text-white py-3 cursor-pointer rounded-lg h-16"
           onclick={Callback::from({
               let navigator = navigator.clone();
               move |_| {
                   navigator.push(&Route::StatCharts { chart_type: ChartType::Day });
               }
           })}
       >
           {"Day"}
       </button>
       </div></div>
       }
}
