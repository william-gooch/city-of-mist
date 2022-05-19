use crate::state::State;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct CharacterCard {
    state: Rc<State>,
    _dispatch: Dispatch<BasicStore<State>>,

    flipped: bool,
}

pub enum CharacterCardMsg {
    State(Rc<State>),
    Flip,
}

impl Component for CharacterCard {
    type Properties = ();
    type Message = CharacterCardMsg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _dispatch: Dispatch::bridge_state(ctx.link().callback(CharacterCardMsg::State)),
            state: Default::default(),
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(character) = &self.state.character {
            let class = format!(
                "card {}character",
                if self.flipped { "card-flipped " } else { "" },
            );

            html! {
                <div class={class}>
                    <div class="card-inner">
                        <div class="card-front">
                            <img alt="Character Card" src="assets/character-card.png" />
                            <h1 class="card-title">{ &character.name }</h1>
                            <h3 class="card-mythos">{ &character.mythos }</h3>
                            <h3 class="card-logos">{ &character.logos }</h3>
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
