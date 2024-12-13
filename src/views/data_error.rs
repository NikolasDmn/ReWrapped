use yew::prelude::*;

#[function_component(DataError)]
pub fn data_error() -> Html {
    html! {
    <div class="upload-guide-container text-xl">
                       <div class="header flex items-center">
            <a class="logo-container w-1/3 flex flex-col items-center mb-20" href="/">
                <img src="assets/logo/logo.svg" alt="logo" class="logo mb-4 w-60" />
                <h2 class="text-4xl text-center"> { "ReWrapped" } </h2>
            </a>
            <div class="title-container w-3/4 ">
                <h1 class="text-8xl text-center"> { "Data Error" } </h1>
            </div>
        </div>

        <p class="text-4xl"> {"We experienced an error while parsing your data. Please ensure you are providing the correct data. If this issue persists do let us know"} </p>
                    </div>
        }
}
