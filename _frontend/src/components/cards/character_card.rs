use dioxus::prelude::*;

use crate::components::app::CHARACTER;

pub fn CharacterCard(cx: Scope) -> Element {
    let character = use_atom_ref(&cx, CHARACTER);

    let flipped = use_state(&cx, || false);
    let class = {
        let is_flipped = *flipped.get();
        format!("card {}character", if is_flipped { "card-flipped " } else { "" })
    };

    cx.render(rsx!(
        div {
            class: format_args!("{}", class),

            div { 
                class: "card-inner",

                div {
                    class: "card-front", 

                    img { alt: "Character Card", src: "assets/character-card.png" }
                    div { class: "card-title", [character.read().name.clone()] }
                    div { class: "card-mythos", [character.read().mythos.clone()] }
                    div { class: "card-logos", [character.read().logos.clone()] }
                    div { class: "flip-button", onclick: |_| flipped.set(true), "Flip" }
                }
                div {
                    class: "card-back",

                    img { alt: "Character Card", src: "assets/com-build-up.png" }
                    div { class: "flip-button", onclick: |_| flipped.set(false), "Flip" }
                }
            }
        }
    ))
}
