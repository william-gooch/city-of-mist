use super::dice_agent::{DiceAgent, DiceMessage};
use super::renderer::Renderer;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged, Dispatched, Dispatcher};

pub enum Msg {
    Noop,
    TriggerRollSeeded(u64),
    Values(Vec<i8>),
}

pub struct DiceOverlay {
    canvas_ref: NodeRef,
    dice_agent: Box<dyn Bridge<DiceAgent>>,
    renderer: Option<Rc<RefCell<Renderer>>>,
    values: Option<Vec<i8>>,
}

impl DiceOverlay {
    pub fn trigger_roll_seeded(&self, seed: u64) {
        if let Some(renderer) = &self.renderer {
            Renderer::roll_dice_seeded(renderer, seed);
        }
    }
}

impl Component for DiceOverlay {
    type Properties = ();
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            dice_agent: DiceAgent::bridge(ctx.link().callback(|msg| match msg {
                DiceMessage::TriggerRollSeeded(seed) => Msg::TriggerRollSeeded(seed),
                _ => Msg::Noop,
            })),
            renderer: None,
            values: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TriggerRollSeeded(seed) => {
                self.trigger_roll_seeded(seed);
                false
            }
            Msg::Values(values) => {
                self.dice_agent.send(DiceMessage::Values(values));
                false
            }
            _ => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.canvas_ref.clone()} id="dice-overlay" />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let on_values = ctx.link().callback(Msg::Values);

            let renderer = Renderer::new(self.canvas_ref.clone(), on_values).setup();
            Renderer::do_loop(&renderer);
            self.renderer = Some(renderer);
        }
    }
}
