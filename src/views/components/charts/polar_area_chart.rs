use serde::Serialize;
use serde_json::to_string;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

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
        let colours = [
            "#189a46", "#209e4b", "#27a14f", "#2ea554", "#34a858", "#3aac5d", "#3fb062", "#44b366",
            "#49b76b", "#4eba70", "#53be74", "#58c279", "#5dc57d", "#62c982", "#67cd87", "#6bd08b",
            "#70d490", "#75d895", "#79db99", "#7edf9e", "#83e3a3", "#88e7a8", "#8ceaac", "#91eeb1",
        ];

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
