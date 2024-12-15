use serde::Serialize;
use serde_json::to_string;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct BarChartData {
    pub name: String,
    pub value: f32,
}

impl BarChartData {
    pub fn convert(value: Vec<(String, f32)>) -> Vec<Self> {
        value
            .into_iter()
            .map(|(name, value)| BarChartData { name, value })
            .collect()
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BarChartProps {
    pub data: Vec<BarChartData>,
}

#[function_component(BarChart)]
pub fn bar_chart(props: &BarChartProps) -> Html {
    use_effect_with(props.clone(), {
        move |data| {
            let json_data = to_string(&data.data).unwrap();
            create_bar_chart(&json_data);
            || ()
        }
    });

    html! {
        <div id="bar-chart" class="w-full h-full"></div>
    }
}

#[wasm_bindgen(module = "/src/views/components/charts/bar_chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "create_bar_chart")]
    pub fn create_bar_chart(data: &str);
}
