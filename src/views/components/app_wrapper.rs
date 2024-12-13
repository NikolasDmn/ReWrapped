use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppWrapperProps {
    pub children: Children,
}

#[function_component]
pub fn AppWrapper(props: &AppWrapperProps) -> Html {
    html! {
        <div class="w-full p-8 flex flex-col items-center h-screen max-h-full overflow-y-hidden">
            <div class="bg-neutral app rounded-xl p-16 h-full overflow-y-auto overflow-x-hidden">
                { for props.children.iter() }
            </div>
        </div>
    }
}
