use super::cube::{Cube, CubeBuilder};
use super::model::Model;
use super::physics::Physics;
use super::shader::{Shader, ShaderBuilder};
use super::texture::Texture;
use rand::SeedableRng;
use rapier3d::{na::*, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use yew::prelude::*;

pub struct Renderer {
    canvas_ref: NodeRef,
    context: Option<WebGl2RenderingContext>,
    shader: Option<Shader>,
    dice_texture: Option<Texture>,
    models: Vec<Cube>,
    physics: Physics,
    cached_value: Option<Vec<i8>>,
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
            dice_texture: None,
            physics,
            cached_value: None,
            on_value,
        }
    }

    pub fn setup(mut self) -> Rc<RefCell<Self>> {
        let canvas = self
            .canvas_ref
            .cast::<web_sys::HtmlCanvasElement>()
            .expect("Node reference is not a canvas element.");
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .expect("No webgl2 context.")
            .dyn_into::<WebGl2RenderingContext>()
            .expect("Couldn't convert webgl2 context to correct type.");
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

            uniform vec4 uColorPrimary;
            uniform vec4 uColorSecondary;
            uniform sampler2D uDiffuse;

            varying vec3 normal;
            varying vec2 uv;

            void main() {
                vec3 sun = normalize(vec3(1.0, 2.0, 3.0));
                float diffuse_factor = abs(dot(sun, normal)) * 0.5 + 0.5;
                vec3 diffuse = diffuse_factor
                    * (texture2D(uDiffuse, uv).x * uColorPrimary.xyz)
                    + (texture2D(uDiffuse, uv).y * uColorSecondary.xyz);
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

        self.dice_texture = Some(Texture::new(&context, "assets/tex-die.png"));

        self.update_models();

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

    fn update_models(&mut self) {
        if let Some(context) = &self.context {
            self.models
                .iter_mut()
                .for_each(|model| model.setup(context));
        }
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
        let values: Vec<i8> = this
            .models
            .iter_mut()
            .filter_map(|model| {
                model.update(time);
                model.value()
            })
            .collect();
        if values.len() == this.models.len() {
            if let Some(mut cached_value) = this.cached_value.as_mut() {
                if *cached_value != values {
                    *cached_value = values.clone();
                    this.on_value.emit(values);
                }
            } else {
                this.cached_value = Some(values.clone());
                this.on_value.emit(values);
            }
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
            let r = 0.3;
            let a = (canvas.width() as Real) / (canvas.height() as Real);
            let p = Matrix4::<Real>::new_orthographic(-r * a, r * a, -r, r, 0.0, 100.0);
            let mvp = p * v * m;
            this.shader.as_ref().unwrap().set_mat4("mvp", &mvp);
            model.set_uniforms(this.shader.as_ref().unwrap());
            model.draw(&context);
        });
    }

    pub fn roll_dice_seeded(this: &Rc<RefCell<Self>>, seed: u64) {
        let this = &mut *this.borrow_mut();
        if let Some(texture) = &this.dice_texture {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let cube_1 = CubeBuilder::default()
                .texture(texture.clone())
                .color_primary(vector!(0.86, 0.82, 0.79, 0.0))
                .color_secondary(vector!(0.3, 0.3, 0.3, 0.0))
                .random_transform(&mut rng)
                .random_linvel(&mut rng)
                .random_angvel(&mut rng)
                .build(this.physics.clone())
                .unwrap();
            let cube_2 = CubeBuilder::default()
                .texture(texture.clone())
                .color_primary(vector!(0.14, 0.18, 0.21, 0.0))
                .color_secondary(vector!(0.7, 0.7, 0.7, 0.0))
                .random_transform(&mut rng)
                .random_linvel(&mut rng)
                .random_angvel(&mut rng)
                .build(this.physics.clone())
                .unwrap();
            this.models = vec![cube_1, cube_2];
            this.update_models();
        }
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
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
