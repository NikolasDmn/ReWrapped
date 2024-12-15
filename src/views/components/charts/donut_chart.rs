use wasm_bindgen::prelude::*;
use yew::prelude::*;

use serde::Serialize;
use serde_json::to_string;

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

#[function_component(PieChart)]
pub fn pie_chart(props: &PieChartProps) -> Html {
    use_effect_with(props.clone(), {
        move |data| {
            let json_data = to_string(&data.data).unwrap();
            show_donut_chart(&json_data);
            || ()
        }
    });
    html! {

    <div id="pie-chart" class="w-full h-full" ></div>

    }
}

#[wasm_bindgen(module = "/src/views/components/charts/donut_chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "show_donut_chart")]
    pub fn show_donut_chart(data: &str);
}
