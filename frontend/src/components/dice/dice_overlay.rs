use web_sys::console::log_1;
use std::cell::RefCell;
use super::renderer::Renderer;
use std::rc::Rc;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_values: Callback<Vec<i8>>,
}

pub struct DiceOverlay {
    canvas_ref: NodeRef,
    renderer: Option<Rc<RefCell<Renderer>>>,
    values: Option<Vec<i8>>,
}

impl Component for DiceOverlay {
    type Properties = Props;
    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            renderer: None,
            values: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onclick = Callback::from(|_| {
            log_1(&"hello".into());
        });

        html! {
            <canvas ref={self.canvas_ref.clone()} id="dice-overlay" {onclick} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let renderer = Renderer::new(self.canvas_ref.clone(), ctx.props().on_values.clone()).setup();
            Renderer::do_loop(&renderer);
            self.renderer = Some(renderer);
        }
    }
}
