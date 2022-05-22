use crate::components::socket::*;
use crate::state::State;
use common::theme::*;
use data::*;
use serde_json::json;
use std::rc::Rc;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
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
    socket: Box<dyn Bridge<SocketConnection>>,

    flipped: bool,
}

pub enum ThemeCardMsg {
    State(Rc<State>),
    Flip,
    UpdateAttention(i8),
    UpdateDegrade(i8),
    Noop,
}

impl ThemeCard {
    fn get_theme<'a>(state: &'a State, card: ThemeCardType) -> &'a Theme {
        match card {
            ThemeCardType::Core(idx) => &state.character.as_ref().unwrap().core_themes[idx],
            ThemeCardType::Crew => state
                .character
                .as_ref()
                .unwrap()
                .crew_theme
                .as_ref()
                .unwrap(),
            ThemeCardType::Extra(idx) => &state.character.as_ref().unwrap().extra_themes[idx],
        }
    }
}

impl Component for ThemeCard {
    type Properties = ThemeCardProps;
    type Message = ThemeCardMsg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            dispatch: Dispatch::bridge_state(ctx.link().callback(ThemeCardMsg::State)),
            socket: SocketConnection::bridge(ctx.link().callback(move |evt| match evt {
                _ => ThemeCardMsg::Noop,
            })),
            state: Default::default(),
            flipped: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ThemeCardMsg::State(state) => {
                self.state = state;
                true
            }
            ThemeCardMsg::Flip => {
                self.flipped = !self.flipped;
                true
            }
            ThemeCardMsg::UpdateAttention(i) => {
                let character_id = self.state.character.as_ref().unwrap().id.unwrap();
                let theme = Self::get_theme(&self.state, ctx.props().card);
                log_1(&"updating attention".into());
                self.socket.send(SocketMessage(
                    "character/modify".to_owned(),
                    json!({
                        "cid": character_id,
                        "character": {
                            "core_themes": [
                                {
                                    "id": theme.id,
                                    "attention": i,
                                }
                            ]
                        }
                    }),
                ));
                false
            }
            ThemeCardMsg::UpdateDegrade(i) => {
                let character_id = self.state.character.as_ref().unwrap().id.unwrap();
                let theme = Self::get_theme(&self.state, ctx.props().card);
                self.socket.send(SocketMessage(
                    "character/modify".to_owned(),
                    json!({
                        "cid": character_id,
                        "character": {
                            "core_themes": [
                                {
                                    "id": theme.id,
                                    "fade_or_crack": i,
                                }
                            ]
                        }
                    }),
                ));
                false
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let None = self.state.character {
            return html! {};
        };

        let state = &self.state;
        let card = ctx.props().card;
        let theme = Self::get_theme(state, card);

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

        let _state = state.clone();
        let onmousedown_attention = ctx.link().callback(move |e: MouseEvent| {
            let theme = Self::get_theme(&_state, card);
            match e.which() {
                1 => {
                    if theme.attention < 3 {
                        log_1(&"should update attention".into());
                        ThemeCardMsg::UpdateAttention(theme.attention + 1)
                    } else {
                        ThemeCardMsg::Noop
                    }
                }
                3 => {
                    if theme.attention > 0 {
                        log_1(&"should update attention".into());
                        ThemeCardMsg::UpdateAttention(theme.attention - 1)
                    } else {
                        ThemeCardMsg::Noop
                    }
                }
                _ => ThemeCardMsg::Noop,
            }
        });

        let _state = state.clone();
        let onmousedown_degrade = ctx.link().callback(move |e: MouseEvent| {
            let theme = Self::get_theme(&_state, card);
            match e.which() {
                1 => {
                    if theme.fade_or_crack < 3 {
                        ThemeCardMsg::UpdateDegrade(theme.fade_or_crack + 1)
                    } else {
                        ThemeCardMsg::Noop
                    }
                }
                3 => {
                    if theme.fade_or_crack > 0 {
                        ThemeCardMsg::UpdateDegrade(theme.fade_or_crack - 1)
                    } else {
                        ThemeCardMsg::Noop
                    }
                }
                _ => ThemeCardMsg::Noop,
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
                {for theme.tags().iter().filter_map(|tag| {
                    match tag {
                        Tag::Power { name, burned } => Some(html! {
                            <li>
                            {name}
                            {if *burned { html! {<span class="burned-indicator">{"⚫"}</span>} } else { html! {<></>} }}
                            </li>
                        }),
                        _ => None,
                    }
                })}
            </ul>
                <ul class="card-weakness-tags">
                {for theme.tags().iter().filter_map(|tag| {
                    match tag {
                        Tag::Weakness { name, invoked } => Some(html! {
                            <li>
                            {name}
                            {if *invoked { html! {<span class="burned-indicator">{"⚫"}</span>} } else { html! {<></>} }}
                            </li>
                        }),
                        _ => None,
                    }
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
