use dioxus::prelude::*;

use super::cards::{CharacterCard, ThemeCard};

pub static CHARACTER: AtomRef<common::character::Character> = |builder| data::example_character();

pub fn App(cx: Scope) -> Element {
    let character = use_atom_ref(&cx, CHARACTER);

    let crew_theme = character.read().crew_theme.clone().unwrap();
    let crew_theme_exists = character.read().crew_theme.clone().is_some();
    
    let core_themes = character.read().core_themes.clone();
    let extra_themes = character.read().extra_themes.clone();

    cx.render(rsx!(
        img { alt: "City of Mist Logo", class: "logo", src: "assets/com-logo.png" }
        div { 
            class: "card-row",

            CharacterCard {}
            if crew_theme_exists { rsx!(cx, ThemeCard { theme: crew_theme }) } else { () }
            extra_themes.iter().map(|theme| rsx!(cx, 
                ThemeCard { theme: theme }
            ))
        }
    ))
}
