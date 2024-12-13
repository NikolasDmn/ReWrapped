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
            show_chart(&json_data);
            || ()
        }
    });
    html! {
     <div class="flex flex-col items-center justify-center h-screen">
      <div class="flex flex-row items-center justify-center w-full">
        <a class="logo-container w-1/3 flex flex-col items-center mb-10" href="/">
          <img src="/assets/logo/logo.svg" alt="logo" class="logo mb-4 w-60" />
          <h2 class="text-4xl text-center"> { "ReWrapped" } </h2>
        </a>
        <p class="text-6xl text-text-base ml-4 text-center w-full">
                   { "Your Top Artists" }
        </p>
      </div>

      <div class="w-full  h-full flex flex-col items-center">
        <h3 class="text-2xl font-medium mb-4 text-gray-700"> { "How much of your favorite artists occupied your year?" } </h3>
        <div id="pie-chart" class="w-full h-full" ></div>
      </div>
    </div>
        }
}

#[wasm_bindgen(module = "/src/views/components/chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "show_chart")]
    pub fn show_chart(data: &str);
}
