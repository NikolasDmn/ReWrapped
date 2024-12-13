mod data_parser;
mod views;
use views::components::app_wrapper::AppWrapper;
use views::components::data_context::DataProvider;
use views::data_error::DataError;
use views::file_upload::FileUploadView;
use views::stats::top_artists::TopArtists;
use views::upload_guide::UploadGuide;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/upload")]
    Upload,
    #[at("/upload-guide")]
    UploadGuide,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/data-error")]
    DataError,
    #[at("/stats/top-artists")]
    TopArtists,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home | Route::Upload => html! {<AppWrapper><FileUploadView/></AppWrapper>},
        Route::UploadGuide => html! {<AppWrapper><UploadGuide/> </AppWrapper>},
        Route::DataError => html! {<AppWrapper><DataError/></AppWrapper>},
        Route::NotFound => todo!(),
        Route::TopArtists => html! {<AppWrapper><TopArtists/></AppWrapper>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
    <BrowserRouter>
        <Switch<Route> render={switch} />
    </BrowserRouter>
    }
}
#[function_component(Root)]
fn root() -> Html {
    html! {
        <DataProvider>
            <App/>
        </DataProvider>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
