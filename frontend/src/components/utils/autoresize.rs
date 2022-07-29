use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AutoresizeProps {
    pub class: String,
    pub value: String,
    pub default_font_size: String,
    pub expected_length: i32,
    pub onchange: Callback<Option<String>>,
}

fn calc_size_mod(value: &str, expected_length: i32) -> f32 {
    f32::powf(1.5, -(2.0 * value.len() as f32) / expected_length as f32)
}

#[function_component(Autoresize)]
pub fn autoresize(props: &AutoresizeProps) -> Html {
    let size_mod = use_state_eq(|| calc_size_mod(&props.value, props.expected_length));

    let onchange = {
        let onchange = props.onchange.clone();

        Callback::from(move |e: html::onchange::Event| {
            let value = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
                .map(|t| t.value());
            onchange.emit(value);
        })
    };

    let onblur = {
        let sm = size_mod.clone();
        let value = props.value.clone();
        let expected_length = props.expected_length.clone();

        Callback::from(move |_e: html::onblur::Event| {
            sm.set(calc_size_mod(&value, expected_length));
        })
    };

    html! {
        <textarea
            id="#textarea"
            class={classes!("autoresize", props.class.clone())}
            value={props.value.clone()}
            {onchange}
            {onblur}
            style={format!("font-size: calc({} * {})", props.default_font_size.clone(), *size_mod)}
        />
    }
}
