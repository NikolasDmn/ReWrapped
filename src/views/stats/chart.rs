use std::time::Duration;
use std::{str::FromStr, thread};

use crate::data_parser::playback_record::PlaybackRecord;
use crate::views::components::charts::bar_chart::{BarChart, BarChartData};
use crate::{
    views::components::{
        charts::donut_chart::{PieChart, PieChartData},
        data_context,
    },
    Route,
};
use gloo::{console::log, timers::future::sleep};
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};

use crate::{data_parser::queries, views::components::data_context::DataContext};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChartType {
    Artists,
    Songs,
    Albums,
    Week,
    Months,
    Day,
    Platform,
    Country,
}

impl ChartType {
    fn get_chart(&self, dt: Vec<(String, f32)>) -> Html {
        match self {
            Self::Artists | Self::Songs | Self::Albums => {
                html! {<PieChart data={from_raw_to_pie_data(dt)}/>}
            }

            Self::Months | Self::Week | Self::Platform | Self::Country => {
                html! {<BarChart data={BarChartData::convert(dt)}/>}
            }
            Self::Day => todo!(),
        }
    }
}
impl std::str::FromStr for ChartType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "artist" | "artists" => Ok(Self::Artists),
            "song" | "songs" => Ok(Self::Songs),
            "album" | "albums" => Ok(Self::Albums),
            "week" | "weeks" => Ok(Self::Week),
            "months" | "month" => Ok(Self::Months),
            "day" | "days" => Ok(Self::Day),
            "platform" | "platforms" => Ok(Self::Platform),
            "country" | "countries" => Ok(Self::Country),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ChartType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Artists => "Artists".to_string(),
                Self::Songs => "Songs".to_string(),
                Self::Albums => "Albums".to_string(),
                Self::Week => "Week".to_string(),
                Self::Months => "Months".to_string(),
                Self::Day => "Day".to_string(),
                Self::Platform => "Platform".to_string(),
                Self::Country => "Country".to_string(),
            }
        )
    }
}
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

#[derive(Properties, PartialEq, Clone)]
pub struct ChartViewProps {
    pub chart_type: ChartType,
}

#[function_component(ChartView)]
pub fn chart_view(props: &ChartViewProps) -> Html {
    let data_context = use_context::<DataContext>().unwrap();
    let data: UseStateHandle<Vec<(String, f32)>> = use_state(|| vec![]);
    let navigator = use_navigator().unwrap();
    let loading = use_state(|| true);

    use_effect_with((), {
        let data = data.clone();
        let loading = loading.clone();
        let chart_type = props.chart_type.clone();
        move |_| {
            if data_context.inner.is_empty() {
                navigator.push(&Route::Upload);
                return;
            }
            let dt = &data_context.inner;
            data.set(match chart_type {
                ChartType::Albums => queries::get_top_albums_percentages(dt, 4.0, 5),
                ChartType::Artists => queries::get_top_artists_percentages(dt, 4.0, 5),
                ChartType::Songs => queries::get_top_songs_percentages(dt, 4.0, 5),
                ChartType::Week => queries::get_day_distribution(dt),
                ChartType::Months => queries::get_months_distribution(dt),
                ChartType::Platform => queries::get_top_platforms(dt),
                ChartType::Country => queries::get_top_countries(dt),
                ChartType::Day => todo!(),
            });
            loading.set(false);
        }
    });
    html! {
    <div class="flex flex-col items-center justify-center h-full">
      <div class="flex flex-row items-center justify-center w-full">
        <a class="logo-container w-1/3 flex flex-col items-center mb-10" href="/">
          <img src="/assets/logo/logo.svg" alt="logo" class="logo mb-4 w-60" />
          <h2 class="text-4xl text-center"> { "ReWrapped" } </h2>
        </a>
        <p class="text-6xl text-text-base ml-4 text-center w-full">
                   { format!("Your Top {}", props.chart_type.to_string()) }
        </p>
      </div>

      <div class="w-full  h-full flex flex-col items-center">
        <h3 class="text-2xl font-medium mb-4 text-gray-700"> { format!("How much of your favorite {} occupied your year?", props.chart_type.to_string().to_lowercase()) } </h3>
            if *loading {
                <span class="loading loading-dots loading-lg"></span>
            }
            else {
                {props.chart_type.get_chart((*data).clone())}
            }
        </div>
    </div>
    }
}
