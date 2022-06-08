use crate::components::{
    socket::*,
    utils::autoresize::Autoresize,
};
use crate::state::State;
use std::rc::Rc;
use serde_json::json;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yewdux::prelude::*;

pub struct CharacterCard {
    state: Rc<State>,
    _dispatch: Dispatch<BasicStore<State>>,
    socket: Box<dyn Bridge<SocketConnection>>,

    flipped: bool,
}

pub enum CharacterCardMsg {
    State(Rc<State>),
    Flip,
    SetName(String),
    SetMythos(String),
    SetLogos(String),
}

impl CharacterCard {
    fn send_update(&mut self, character: serde_json::Value) {
        let character_id = self.state.character.as_ref().unwrap().id.unwrap();
        self.socket.send(SocketMessage(
            "character/modify".to_owned(),
            json!({
                "cid": character_id,
                "character": character,
            }),
        ));
    }
}

impl Component for CharacterCard {
    type Properties = ();
    type Message = CharacterCardMsg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _dispatch: Dispatch::bridge_state(ctx.link().callback(CharacterCardMsg::State)),
            state: Default::default(),
            socket: SocketConnection::bridge(ctx.link().batch_callback(move |_| None)),
            flipped: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CharacterCardMsg::State(state) => {
                self.state = state;
                true
            }
            CharacterCardMsg::Flip => {
                self.flipped = !self.flipped;
                true
            }
            CharacterCardMsg::SetName(name) => {
                self.send_update(json!({
                    "name": name
                }));
                false
            }
            CharacterCardMsg::SetMythos(mythos) => {
                self.send_update(json!({
                    "mythos": mythos
                }));
                false
            }
            CharacterCardMsg::SetLogos(logos) => {
                self.send_update(json!({
                    "logos": logos
                }));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(character) = &self.state.character {
            let class = format!(
                "card {}character",
                if self.flipped { "card-flipped " } else { "" },
            );

            let onchange_name = ctx.link().batch_callback(|e: Option<String>| {
                log_1(&"onchange title".into());
                e.map(|v| CharacterCardMsg::SetName(v))
            });

            let onchange_mythos = ctx.link().batch_callback(|e: Option<String>| {
                log_1(&"onchange mythos".into());
                e.map(|v| CharacterCardMsg::SetMythos(v))
            });

            let onchange_logos = ctx.link().batch_callback(|e: Option<String>| {
                log_1(&"onchange logos".into());
                e.map(|v| CharacterCardMsg::SetLogos(v))
            });

            html! {
                <div class={class}>
                    <div class="card-inner">
                        <div class="card-front">
                            <img alt="Character Card" src="assets/character-card.png" />
                            <Autoresize class="card-title" default_font_size="5vh" expected_length=30 value={character.name.clone()} onchange={onchange_name} />
                            <Autoresize class="card-mythos" default_font_size="3vh" expected_length=200 value={character.mythos.clone()} onchange={onchange_mythos} />
                            <Autoresize class="card-logos" default_font_size="3vh" expected_length=200 value={character.logos.clone()} onchange={onchange_logos} />
                            <div class="flip-button" onclick={ctx.link().callback(|_| CharacterCardMsg::Flip)}>{ "Flip" }</div>
                        </div>
                        <div class="card-back">
                            <img alt="Character Card" src="assets/com-build-up.png" />
                            <div class="flip-button" onclick={ctx.link().callback(|_| CharacterCardMsg::Flip)}>{ "Flip" }</div>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }
}
