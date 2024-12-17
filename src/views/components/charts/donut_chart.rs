use wasm_bindgen::prelude::*;
use yew::prelude::*;

use serde::Serialize;
use serde_json::to_string;

use crate::views::stats::chart::get_gradient;

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct PieChartData {
    pub name: String,
    pub value: f32,
    pub color: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PieChartProps {
    pub data: Vec<PieChartData>,
}
impl PieChartData {
    pub fn convert(data: Vec<(String, f32)>) -> Vec<Self> {
        let colours = get_gradient("#10b981", "#4ade80", data.len());
        data.into_iter()
            .zip(colours)
            .map(|((name, value), color)| Self {
                name,
                value,
                color: color.to_string(),
            })
            .collect()
    }
}

#[function_component(PieChart)]
pub fn pie_chart(props: &PieChartProps) -> Html {
    use_effect_with(props.clone(), {
        move |data| {
            let json_data = to_string(&data.data).unwrap();
            create_donut_chart(&json_data);
            || ()
        }
    });
    html! {

    <div id="pie-chart" class="w-full h-full" ></div>

    }
}

#[wasm_bindgen(module = "/src/views/components/charts/donut_chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "create_donut_chart")]
    pub fn create_donut_chart(data: &str);
}
