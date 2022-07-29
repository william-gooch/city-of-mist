use data::HasThemeDescriptor;
use dioxus::prelude::*;

use crate::components::app::CHARACTER;

use common::theme::ThemeType;

#[derive(PartialEq, Eq)]
pub enum ThemeRef {
    CoreTheme(i8),
    CrewTheme,
    ExtraTheme(i8),
}

#[derive(PartialEq, Props)]
pub struct ThemeCardProps {
    theme_ref: ThemeRef,
}

pub fn ThemeCard(cx: Scope, props: ThemeCardProps) -> Element {
    let character = use_atom_ref(&cx, CHARACTER);

    let flipped = use_state(&cx, || false);
    let class = { format!(
        "card {}theme theme-{}",
        if *flipped.get() { "card-flipped " } else { "" },
        match props.theme.get().descriptor().theme_type() {
            ThemeType::Mythos => "mythos",
            ThemeType::Logos => "logos",
            ThemeType::Crew => "crew",
            ThemeType::Extra => "extra",
        }
    ) };

    let front_image = { match props.theme.get().descriptor().theme_type() {
        ThemeType::Mythos | ThemeType::Logos => "assets/character-theme.png",
        ThemeType::Crew | ThemeType::Extra => "assets/crew-theme.png",
    } };

    let back_image = { match props.theme.get().descriptor().theme_type() {
        ThemeType::Mythos | ThemeType::Logos => "assets/character-theme-flipside.png",
        ThemeType::Crew | ThemeType::Extra => "assets/crew-improvements.png",
    } };

    let themebook = { props.theme.get().descriptor().name().clone() };
    let themebook = { props.theme.get().title().clone() };

    cx.render(rsx!(
        div {
            class: class,

            div {
                class: "card-inner",

                div {
                    class: "card-front",

                    img { alt: "Character Card", src: (*front_image.get()) }
                    h5 { class: "card-type", (*themebook.get()) }
                    div { class: "card-title", (*title.get()) }
                    div { class: "flip-button", onclick: |_| flipped.set(true), "Flip" }
                }
                div {
                    class: "card-back",

                    img { alt: "Character Card", src: (*back_image.get()) }
                    div { class: "flip-button", on:click: |_| flipped.set(false), "Flip" }
                }
            }
        }
    ))
}
