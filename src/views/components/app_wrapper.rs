use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppWrapperProps {
    pub children: Children,
}

#[function_component]
pub fn AppWrapper(props: &AppWrapperProps) -> Html {
    html! {
        <div class="w-full p-8 flex flex-col items-center h-screen max-h-full overflow-y-hidden  backdrop-blur-lg">
            <div class="bg-neutral bg-opacity-50 app rounded-xl p-16 h-full overflow-y-auto overflow-x-hidden shadow-4xl backdrop-blur-3xl shadow-inner">
                { for props.children.iter() }
            </div>
        </div>
    }
}
