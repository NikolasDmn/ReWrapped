use std::thread;
use std::time::Duration;

use crate::{
    views::components::{
        chart::{PieChart, PieChartData},
        data_context,
    },
    Route,
};
use gloo::{console::log, timers::future::sleep};
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};

use crate::{data_parser::queries, views::components::data_context::DataContext};

fn from_raw_to_pie_data(data: Vec<(String, f32)>) -> Vec<PieChartData> {
    log!(format!("{:?}", data));
    let colours = [
        "#91eeb2", "#65e793", "#39e074", "#1fc65a", "#189a46", "#116e32",
    ];
    data.into_iter()
        .zip(colours)
        .map(|((name, value), color)| PieChartData {
            name,
            value,
            color: color.to_string(),
        })
        .collect()
}
#[function_component(TopArtists)]
pub fn top_artsits() -> Html {
    let data_context = use_context::<DataContext>().unwrap();
    let data: UseStateHandle<Vec<PieChartData>> = use_state(|| vec![]);
    let navigator = use_navigator().unwrap();
    let loading = use_state(|| true);
    use_effect_with((), {
        let data = data.clone();
        let loading = loading.clone();
        move |_| {
            if data_context.inner.is_empty() {
                navigator.push(&Route::Upload);
                return;
            }
            data.set(from_raw_to_pie_data(queries::get_top_artists_percentages(
                &data_context.inner,
                4.0,
                5,
            )));
            loading.set(false);
        }
    });
    html! {
    if *loading {
        <span class="loading loading-dots loading-lg"></span>
    }
    else {
        <PieChart data={(*data).clone()}/>
        }
    }
}
