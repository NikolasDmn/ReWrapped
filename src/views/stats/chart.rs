use std::time::Duration;
use std::{str::FromStr, thread};

use crate::data_parser::playback_record::PlaybackRecord;
use crate::views::components::charts::bar_chart::{BarChart, BarChartData};
use crate::views::components::charts::polar_area_chart::{PolarAreaChart, PolarAreaChartData};
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

pub fn get_gradient(start_color: &str, end_color: &str, steps: usize) -> Vec<String> {
    let start_r = u8::from_str_radix(&start_color[1..3], 16).unwrap();
    let start_g = u8::from_str_radix(&start_color[3..5], 16).unwrap();
    let start_b = u8::from_str_radix(&start_color[5..7], 16).unwrap();
    let end_r = u8::from_str_radix(&end_color[1..3], 16).unwrap();
    let end_g = u8::from_str_radix(&end_color[3..5], 16).unwrap();
    let end_b = u8::from_str_radix(&end_color[5..7], 16).unwrap();
    (0..steps)
        .map(|i| {
            let t = i as f32 / (steps - 1) as f32;
            let r = (start_r as f32 + t * (end_r as f32 - start_r as f32)) as u8;
            let g = (start_g as f32 + t * (end_g as f32 - start_g as f32)) as u8;
            let b = (start_b as f32 + t * (end_b as f32 - start_b as f32)) as u8;
            format!("#{:02X}{:02X}{:02X}", r, g, b)
        })
        .collect()
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChartType {
    Artists,
    Songs,
    Albums,
    Week,
    Months,
    Days,
    Day,
    Platform,
    Country,
}

impl ChartType {
    fn get_chart(&self, dt: Vec<(String, f32)>) -> Html {
        match self {
            Self::Artists | Self::Songs | Self::Albums | Self::Days => {
                html! {<PieChart data={PieChartData::convert(dt)}/>}
            }

            Self::Months | Self::Week | Self::Platform | Self::Country => {
                html! {<BarChart data={BarChartData::convert(dt)}/>}
            }
            Self::Day => {
                html! {<PolarAreaChart data={PolarAreaChartData::convert(dt)}/>}
            }
        }
    }
    fn get_message(&self) -> String {
        match self {
            Self::Artists | Self::Songs | Self::Albums => format!(
                "How much of your favorite {} occupied your year?",
                self.to_string().to_lowercase()
            ),
            Self::Week => format!("Which day of the week did you listen to music the most?"),
            Self::Months => format!("Which month did you listen to music the most?"),
            Self::Day => format!("Which hour of the day did you listen to music the most?"),
            Self::Country => format!("From where did you listen to music the most?"),
            Self::Platform => format!("Which platform did you use to listen to music the most?"),
            Self::Days => format!("Which days did you listen to music the most this year?"),
        }
    }
    fn get_title(&self) -> String {
        match self {
            Self::Artists | Self::Songs | Self::Albums | Self::Months | Self::Days => {
                format!("Top {}", self)
            }
            Self::Week => format!("Day of the week"),
            Self::Day => format!("Hour of the day"),
            Self::Country => format!("Countries"),
            Self::Platform => format!("Platforms"),
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
            "day" => Ok(Self::Day),
            "days" => Ok(Self::Days),
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
                Self::Days => "Days".to_string(),
                Self::Platform => "Platform".to_string(),
                Self::Country => "Country".to_string(),
                Self::Day => "Day".to_string(),
            }
        )
    }
}
fn from_raw_to_pie_data(data: Vec<(String, f32)>) -> Vec<PieChartData> {
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
                ChartType::Days => queries::get_top_days(dt, 5),
                ChartType::Day => queries::get_hours_of_the_day_distribution(dt),
            });
            loading.set(false);
        }
    });
    html! {
    <div class="flex flex-col items-center justify-center h-full">
      <div class="flex flex-row items-center justify-center w-full">
        <a class="logo-container w-1/4 flex flex-col items-center mb-10" href="/">
          <img src="/assets/logo.svg" alt="logo" class="logo mb-4 w-60" />
          <h2 class="text-4xl text-center"> { "ReWrapped" } </h2>
        </a>
        <p class="text-4xl text-text-base ml-4 text-center w-full">
                   { props.chart_type.get_title() }
        </p>
      </div>

      <div class="w-full  h-full flex flex-col items-center">
        <h3 class="text-xl font-medium mb-4 text-gray-700"> {props.chart_type.get_message()  } </h3>
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
