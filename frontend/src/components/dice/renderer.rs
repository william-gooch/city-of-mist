use super::physics::Physics;
use std::cell::RefCell;
use super::cube::Cube;
use super::model::Model;
use super::shader::{Shader, ShaderBuilder};
use rapier3d::{prelude::*, na::*};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console::log;
use web_sys::console::log_1;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use yew::prelude::*;

pub struct Renderer {
    canvas_ref: NodeRef,
    context: Option<WebGl2RenderingContext>,
    shader: Option<Shader>,
    models: Vec<Cube>,
    physics: Physics,
    on_value: Callback<Vec<i8>>,
}

impl Renderer {
    pub fn new(canvas_ref: NodeRef, on_value: Callback<Vec<i8>>) -> Self {
        let physics = Physics::new();
        Self {
            canvas_ref,
            context: None,
            models: vec![],
            shader: None,
            physics,
            on_value,
        }
    }

    pub fn setup(mut self) -> Rc<RefCell<Self>> {
        let canvas = self
            .canvas_ref
            .cast::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        self.context = Some(context.clone());

        let shader = ShaderBuilder::new()
            .vert(
                r#"
            attribute vec3 vPosition;
            attribute vec3 vNormal;
            attribute vec2 vUV;

            uniform mat4 mvp;

            varying vec3 normal;
            varying vec2 uv;

            void main() {
                gl_Position = mvp * vec4(vPosition, 1.0);
                normal = vNormal;
                uv = vUV;
            }
            "#,
            )
            .frag(
                r#"
            precision mediump float;

            uniform sampler2D uDiffuse;

            varying vec3 normal;
            varying vec2 uv;

            void main() {
                vec3 sun = normalize(vec3(1.0, 2.0, 3.0));
                float diffuse_factor = abs(dot(sun, normal)) * 0.5 + 0.5;
                vec3 diffuse = diffuse_factor * texture2D(uDiffuse, uv).xyz;
                float transparency = 1.0;
                vec3 color = diffuse;
                gl_FragColor = vec4(color, transparency);
            }
            "#,
            )
            .build(&context)
            .unwrap();
        shader.activate();
        self.shader = Some(shader);

        self.models
            .iter_mut()
            .for_each(|model| model.setup(&context));

        let rc_self = Rc::new(RefCell::new(self));
        {
            let rc_self = rc_self.clone();
            let on_resize = Closure::wrap(Box::new(move || {
                Renderer::resize(&rc_self);
            }) as Box<dyn FnMut()>);
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())
                .unwrap();
            on_resize.forget();
        }
        Renderer::resize(&rc_self);

        rc_self
    }

    fn resize(_this: &Rc<RefCell<Self>>) {
        let this = _this.borrow();
        let canvas = this
            .canvas_ref
            .cast::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let context = this.context.as_ref().unwrap();

        let display_width = canvas.client_width();
        let display_height = canvas.client_height();

        canvas.set_width(display_width.try_into().unwrap());
        canvas.set_height(display_height.try_into().unwrap());
        context.viewport(0, 0, display_width, display_height);

        Renderer::draw(_this);
    }

    fn update(this: &Rc<RefCell<Self>>, time: f32) {
        let mut this = this.borrow_mut();
        let values: Vec<i8> = this.models.iter_mut().filter_map(|model| {
            model.update(time);
            model.value()
        }).collect();
        if values.len() == this.models.len() {
            this.on_value.emit(values);
        }
        this.physics.tick();
    }

    fn draw(this: &Rc<RefCell<Self>>) {
        let this = this.borrow();
        let canvas = this
            .canvas_ref
            .cast::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let context = this.context.as_ref().unwrap();

        context.clear_color(0.0, 0.0, 0.0, 0.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        this.models.iter().for_each(|model| {
            let m = model.get_matrix();
            let v = Isometry3::<Real>::look_at_rh(
                &Point3::new(0.0, 0.5, 0.0),
                &Point3::new(0.0, 0.0, 0.0),
                &Vector3::z(),
            )
            .to_matrix();
            // let p = Perspective3::<Real>::new(
            //     (canvas.width() as Real) / (canvas.height() as Real),
            //     45.0,
            //     0.01,
            //     1.0,
            // )
            // .to_homogeneous();
            let r = 0.3;
            let a = (canvas.width() as Real) / (canvas.height() as Real);
            let p = Matrix4::<Real>::new_orthographic(
                -r * a, r * a,
                -r, r,
                0.0, 100.0,
            );
            let mvp = p * v * m;
            this.shader.as_ref().unwrap().set_mat4("mvp", &mvp);
            model.draw(&context);
        });
    }

    pub fn roll_dice(this: &Rc<RefCell<Self>>)  {
        let this = &mut *this.borrow_mut();
        this.models = vec![Cube::new(this.physics.clone()), Cube::new(this.physics.clone())];
    }

    pub fn do_loop(this: &Rc<RefCell<Self>>) {
        let f = Rc::new(RefCell::new(None));
        let initial_time = js_sys::Date::now();
        let current_time = Rc::new(RefCell::new(js_sys::Date::now()));
        let g = f.clone();
        let s = this.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let updated_time = js_sys::Date::now();
            let mut current_time = current_time.borrow_mut();
            let total_time: f32 = (updated_time - initial_time) as f32 / 1000.0;
            let elapsed_time: f32 = (updated_time - *current_time) as f32;

            if elapsed_time > 1000.0 / 60.0 {
                *current_time = updated_time;
                Renderer::update(&s, total_time);
                Renderer::draw(&s);
            }

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
