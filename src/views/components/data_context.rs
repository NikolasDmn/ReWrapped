use yew::prelude::*;

use crate::data_parser::playback_record::PlaybackRecord;

#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub inner: Vec<PlaybackRecord>,
}

impl Reducible for Data {
    type Action = Vec<PlaybackRecord>;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Self { inner: action }.into()
    }
}
pub type DataContext = UseReducerHandle<Data>;

#[derive(Properties, Debug, PartialEq)]
pub struct DataProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn DataProvider(props: &DataProviderProps) -> Html {
    let data = use_reducer(|| Data { inner: vec![] });

    html! {
        <ContextProvider<DataContext> context={data}>
            {props.children.clone()}
        </ContextProvider<DataContext>>
    }
}
