use super::card::*;
use super::dice::{DiceAgent, DiceMessage, DiceOverlay};
use super::socket::*;
use crate::state::State;
use serde_json::json;
use std::rc::Rc;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};
use yewdux::prelude::{BasicStore, Dispatch};

pub struct App {
    _dispatch: Dispatch<BasicStore<State>>,
    socket: Box<dyn Bridge<SocketConnection>>,
    dice_agent: Box<dyn Bridge<DiceAgent>>,
    state: Rc<State>,
    dice_values: Option<Vec<i8>>,
}

pub enum AppMsg {
    Noop,
    State(Rc<State>),
    TriggerRoll,
    TriggerRollSeeded(u64),
    DiceValues(Vec<i8>),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _dispatch: Dispatch::bridge_state(ctx.link().callback(AppMsg::State)),
            socket: SocketConnection::bridge(ctx.link().callback(move |evt| {
                log_1(&format!("{:?}", evt).into());
                match evt {
                    SocketEvent::DiceRoll { seed } => AppMsg::TriggerRollSeeded(seed),
                    _ => AppMsg::Noop,
                }
            })),
            dice_agent: DiceAgent::bridge(ctx.link().callback(|msg| match msg {
                DiceMessage::Values(values) => AppMsg::DiceValues(values),
                _ => AppMsg::Noop,
            })),
            state: Default::default(),
            dice_values: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Noop => false,
            AppMsg::State(state) => {
                self.state = state;
                true
            }
            AppMsg::TriggerRoll => {
                self.socket
                    .send(SocketMessage("dice".to_owned(), json!({})));
                false
            }
            AppMsg::TriggerRollSeeded(seed) => {
                self.dice_agent.send(DiceMessage::TriggerRollSeeded(seed));
                false
            }
            AppMsg::DiceValues(values) => {
                self.dice_values = Some(values);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_logo_click = ctx.link().callback(|_| AppMsg::TriggerRoll);

        if let Some(character) = &self.state.character {
            html! {
                <>
                    <img alt="City of Mist Logo" class="logo" src="assets/com-logo.png" onclick={on_logo_click} />
                    {
                        if let Some(vs) = &self.dice_values {
                            html! { for vs.iter().map(|v| {
                                html! { <p>{v}</p> }
                            }) }
                        } else {
                            html! {}
                        }
                    }
                    <div class="card-row">
                        <CharacterCard />
                        <ThemeCard card={ThemeCardType::Crew} />
                        { for character.extra_themes.iter().enumerate().map(|(i, theme)| {
                            html! {
                                <ThemeCard
                                    key={theme.title.to_string()}
                                    card={ThemeCardType::Extra(i)}
                                />
                            }
                        }) }
                    </div>
                    <div class="card-row">
                        { for character.core_themes.iter().enumerate().map(|(i, theme)| {
                            html! {
                                <ThemeCard
                                    key={theme.title.to_string()}
                                    card={ThemeCardType::Core(i)}
                                />
                            }
                        }) }
                    </div>
                    <DiceOverlay />
                </>
            }
        } else {
            html! {}
        }
    }
}
