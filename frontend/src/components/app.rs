use super::card::*;
use super::dice::DiceOverlay;
use crate::state::State;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct App {
    _dispatch: Dispatch<BasicStore<State>>,
    state: Rc<State>,
    dice_values: Option<Vec<i8>>,
}

pub enum AppMsg {
    State(Rc<State>),
    DiceValues(Vec<i8>),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _dispatch: Dispatch::bridge_state(ctx.link().callback(AppMsg::State)),
            state: Default::default(),
            dice_values: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::State(state) => {
                self.state = state;
                true
            },
            AppMsg::DiceValues(values) => {
                self.dice_values = Some(values);
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_dice_values = ctx.link().callback(|values| AppMsg::DiceValues(values));
        
        html! {
            <>
                <img alt="City of Mist Logo" class="logo" src="assets/com-logo.png" />
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
                    { for self.state.character.extra_themes.iter().enumerate().map(|(i, theme)| {
                        html! {
                            <ThemeCard
                                key={theme.title.to_string()}
                                card={ThemeCardType::Extra(i)}
                            />
                        }
                    }) }
                </div>
                <div class="card-row">
                    { for self.state.character.core_themes.iter().enumerate().map(|(i, theme)| {
                        html! {
                            <ThemeCard
                                key={theme.title.to_string()}
                                card={ThemeCardType::Core(i)}
                            />
                        }
                    }) }
                </div>
                <DiceOverlay on_values={on_dice_values} />
            </>
        }
    }
}
