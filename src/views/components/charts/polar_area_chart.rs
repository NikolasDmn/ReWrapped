use serde::Serialize;
use serde_json::to_string;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::views::stats::chart::get_gradient;

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct PolarAreaChartData {
    pub name: String,
    pub value: f32,
    pub color: String,
}

impl PolarAreaChartData {
    pub fn convert(data: Vec<(String, f32)>) -> Vec<Self> {
        let mut data = data.clone();
        data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let colours = get_gradient("#10b981", "#4ade80", data.len());
        let mut data: Vec<Self> = data
            .into_iter()
            .zip(colours)
            .map(|((name, value), color)| Self {
                name,
                value,
                color: color.to_string(),
            })
            .collect();
        data.sort_by(|a, b| {
            b.name
                .parse::<u32>()
                .unwrap_or(0)
                .partial_cmp(&a.name.parse::<u32>().unwrap_or(0))
                .unwrap()
        });
        data.iter_mut()
            .for_each(|item| item.name = format!("{}:00", item.name));
        data.reverse();
        data
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct PolarAreaChartProps {
    pub data: Vec<PolarAreaChartData>,
}

#[function_component(PolarAreaChart)]
pub fn bar_chart(props: &PolarAreaChartProps) -> Html {
    use_effect_with(props.clone(), {
        move |data| {
            let json_data = to_string(&data.data).unwrap();
            create_polar_area_chart(&json_data);
            || ()
        }
    });

    html! {
        <div id="polar-area-chart" class="w-full h-full"></div>
    }
}

#[wasm_bindgen(module = "/src/views/components/charts/polar_area_chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "create_polar_area_chart")]
    pub fn create_polar_area_chart(data: &str);
}
