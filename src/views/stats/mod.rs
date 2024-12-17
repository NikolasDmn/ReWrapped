use chart::ChartType;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};

use crate::Route;

pub mod chart;

#[function_component(StatsHome)]
pub fn stats_home() -> Html {
    let navigator = use_navigator().unwrap();

    // Define a collection of chart types and their respective labels
    let chart_buttons = vec![
        (ChartType::Artists, "Artists"),
        (ChartType::Songs, "Songs"),
        (ChartType::Albums, "Albums"),
        (ChartType::Week, "Week"),
        (ChartType::Months, "Months"),
        (ChartType::Platform, "Platform"),
        (ChartType::Country, "Country"),
        (ChartType::Days, "Days"),
        (ChartType::Day, "Day"),
    ];

    html! {
        <div>
    <div class="header flex items-center">
            <a class="logo-container w-1/3 flex flex-col items-center mb-20" href="/">
                <img src="assets/logo.svg" alt="logo" class="logo mb-4 w-60" />
                <h2 class="text-3xl text-center"> { "ReWrapped" } </h2>
            </a>
            <div class="title-container w-3/4 ">
                <h1 class="text-4xl text-center"> { "Statistics" } </h1>
            </div>
        </div>
            <div class="grid  gap-4 sm:grid-cols-2 sm:auto-rows-fr">
                // Generate buttons dynamically using a map and a loop
                {for chart_buttons.into_iter().map(|(chart_type, label)| {
                    let navigator = navigator.clone();
                    html! {
                        <button
                            class="mbtn text-2xl font-semibold  h-16 w-40"
                            onclick={Callback::from(move |_| {
                                navigator.push(&Route::StatCharts { chart_type: chart_type.clone() });
                            })}
                        >
                            {label}
                        </button>
                    }
                })}
            </div>
        </div>
    }
}
