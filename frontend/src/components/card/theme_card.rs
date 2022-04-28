use crate::state::State;
use common::theme::*;
use data::*;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum ThemeCardType {
    Core(usize),
    Crew,
    Extra(usize),
}

#[derive(Clone, Properties, PartialEq)]
pub struct ThemeCardProps {
    pub card: ThemeCardType,
}

pub struct ThemeCard {
    state: Rc<State>,
    dispatch: Dispatch<BasicStore<State>>,

    flipped: bool,
}

pub enum ThemeCardMsg {
    State(Rc<State>),
    Flip,
}

impl Component for ThemeCard {
    type Properties = ThemeCardProps;
    type Message = ThemeCardMsg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            dispatch: Dispatch::bridge_state(ctx.link().callback(ThemeCardMsg::State)),
            state: Default::default(),
            flipped: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ThemeCardMsg::State(state) => {
                self.state = state;
                true
            }
            ThemeCardMsg::Flip => {
                self.flipped = !self.flipped;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let card = ctx.props().card;
        let theme = match card {
            ThemeCardType::Core(idx) => &self.state.character.core_themes[idx],
            ThemeCardType::Crew => &self.state.character.crew_theme,
            ThemeCardType::Extra(idx) => &self.state.character.extra_themes[idx],
        };

        let class = format!(
            "card {}theme theme-{}",
            if self.flipped { "card-flipped " } else { "" },
            if *theme.descriptor().theme_type() == ThemeType::Mythos {
                "mythos"
            } else {
                "logos"
            }
        );

        let front_image = match theme.descriptor().theme_type() {
            ThemeType::Mythos | ThemeType::Logos => html! {
                <img alt="Theme Card" src="assets/character-theme.png" />
            },
            ThemeType::Crew | ThemeType::Extra => html! {
                <img alt="Theme Card" src="assets/crew-theme.png" />
            },
        };

        let back_image = match theme.descriptor().theme_type() {
            ThemeType::Mythos | ThemeType::Logos => html! {
                <img alt="Theme Card" src="assets/character-theme-flipside.png" />
            },
            ThemeType::Crew | ThemeType::Extra => html! {
                <img alt="Theme Card" src="assets/crew-improvements.png" />
            },
        };

        let onmousedown_attention =
            self.dispatch
                .reduce_callback_with(move |state, e: MouseEvent| {
                    let mut theme = match card {
                        ThemeCardType::Core(idx) => &mut state.character.core_themes[idx],
                        ThemeCardType::Crew => &mut state.character.crew_theme,
                        ThemeCardType::Extra(idx) => &mut state.character.extra_themes[idx],
                    };
                    match e.which() {
                        1 => {
                            if theme.attention < 3 {
                                theme.attention += 1
                            }
                        }
                        3 => {
                            if theme.attention > 0 {
                                theme.attention -= 1
                            }
                        }
                        _ => (),
                    }
                });

        let onmousedown_degrade =
            self.dispatch
                .reduce_callback_with(move |state, e: MouseEvent| {
                    let mut theme = match card {
                        ThemeCardType::Core(idx) => &mut state.character.core_themes[idx],
                        ThemeCardType::Crew => &mut state.character.crew_theme,
                        ThemeCardType::Extra(idx) => &mut state.character.extra_themes[idx],
                    };
                    match e.which() {
                        1 => {
                            if theme.fade_or_crack < 3 {
                                theme.fade_or_crack += 1
                            }
                        }
                        3 => {
                            if theme.fade_or_crack > 0 {
                                theme.fade_or_crack -= 1
                            }
                        }
                        _ => (),
                    }
                });

        html! {
            <div class={class}>
                <div class="card-inner">
                <div class="card-front">
                {front_image}
            <h5 class="card-type">{ &theme.descriptor().name() }</h5>
                <h1 class="card-title">{ &theme.title() }</h1>
                <div class="card-attention" onmousedown={onmousedown_attention}>
                {for (1..*theme.attention()+1).map(|_| html! { <span class="card-tick" /> })}
            </div>
                <div class="card-degrade" onmousedown={onmousedown_degrade}>
                {for (1..*theme.fade_or_crack()+1).map(|_| html! { <span class="card-tick" /> })}
            </div>
                <h3 class="card-phrase">{ &theme.mystery_or_identity() }</h3>
                <ul class="card-power-tags">
                {for theme.tags().iter().filter_map(|tag| match tag {
                                                                        Tag::Power { name, burned } => Some(html! {
                                                                            <li>
                                                                            {name}
                                                                            {if *burned { html! {<span class="burned-indicator">{"⚫"}</span>} } else { html! {<></>} }}
                                                                            </li>
                                                                        }),
                                                                        _ => None,
                                                                    })}
            </ul>
                <ul class="card-weakness-tags">
                {for theme.tags().iter().filter_map(|tag| match tag {
                                                                        Tag::Weakness { name, invoked } => Some(html! {
                                                                            <li>
                                                                            {name}
                                                                            {if *invoked { html! {<span class="burned-indicator">{"⚫"}</span>} } else { html! {<></>} }}
                                                                            </li>
                                                                        }),
                                                                        _ => None,
                                                                    })}
            </ul>
                <div class="flip-button" onclick={ctx.link().callback(|_| ThemeCardMsg::Flip)}>{ "Flip" }</div>
                </div>
                <div class="card-back">
                {back_image}
            <div class="flip-button" onclick={ctx.link().callback(|_| ThemeCardMsg::Flip)}>{ "Flip" }</div>
                </div>
                </div>
                </div>
        }
    }
}
