use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub button_ref: NodeRef,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let base_classes = "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-emerald-600 text-slate-100 hover:bg-emerald-600/90";
    let combined_classes = if props.class.is_empty() {
        base_classes.to_string()
    } else {
        format!("{} {}", base_classes, props.class)
    };

    html! {
        <button
            class={combined_classes}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
            ref={props.button_ref.clone()}>
            { for props.children.iter() }
        </button>
    }
}
