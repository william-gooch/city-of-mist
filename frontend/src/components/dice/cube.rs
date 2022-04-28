use web_sys::console::log_3;
use super::model::Model;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlImageElement, WebGl2RenderingContext, WebGlBuffer, WebGlTexture,
};
use super::physics::Physics;
use rapier3d::prelude::*;
use rapier3d::na::*;
use crate::utils::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
static CUBE_VERTICES: [f32; 8 * 36] = [
    //   position           normal               uv
    -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,  0.0 / 3.0, 0.0 / 2.0,
     0.5, -0.5, -0.5,   0.0,  0.0, -1.0,  1.0 / 3.0, 0.0 / 2.0,
     0.5,  0.5, -0.5,   0.0,  0.0, -1.0,  1.0 / 3.0, 1.0 / 2.0,
     0.5,  0.5, -0.5,   0.0,  0.0, -1.0,  1.0 / 3.0, 1.0 / 2.0,
    -0.5,  0.5, -0.5,   0.0,  0.0, -1.0,  0.0 / 3.0, 1.0 / 2.0,
    -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,  0.0 / 3.0, 0.0 / 2.0,

    -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,  2.0 / 3.0, 2.0 / 2.0,
     0.5, -0.5,  0.5,   0.0,  0.0,  1.0,  3.0 / 3.0, 2.0 / 2.0,
     0.5,  0.5,  0.5,   0.0,  0.0,  1.0,  3.0 / 3.0, 1.0 / 2.0,
     0.5,  0.5,  0.5,   0.0,  0.0,  1.0,  3.0 / 3.0, 1.0 / 2.0,
    -0.5,  0.5,  0.5,   0.0,  0.0,  1.0,  2.0 / 3.0, 1.0 / 2.0,
    -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,  2.0 / 3.0, 2.0 / 2.0,

    -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,  1.0 / 3.0, 0.0 / 2.0,
    -0.5,  0.5, -0.5,  -1.0,  0.0,  0.0,  1.0 / 3.0, 1.0 / 2.0,
    -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,  2.0 / 3.0, 1.0 / 2.0,
    -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,  2.0 / 3.0, 1.0 / 2.0,
    -0.5, -0.5,  0.5,  -1.0,  0.0,  0.0,  2.0 / 3.0, 0.0 / 2.0,
    -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,  1.0 / 3.0, 0.0 / 2.0,

     0.5,  0.5,  0.5,   1.0,  0.0,  0.0,  2.0 / 3.0, 1.0 / 2.0,
     0.5,  0.5, -0.5,   1.0,  0.0,  0.0,  2.0 / 3.0, 2.0 / 2.0,
     0.5, -0.5, -0.5,   1.0,  0.0,  0.0,  1.0 / 3.0, 2.0 / 2.0,
     0.5, -0.5, -0.5,   1.0,  0.0,  0.0,  1.0 / 3.0, 2.0 / 2.0,
     0.5, -0.5,  0.5,   1.0,  0.0,  0.0,  1.0 / 3.0, 1.0 / 2.0,
     0.5,  0.5,  0.5,   1.0,  0.0,  0.0,  2.0 / 3.0, 1.0 / 2.0,

    -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,  2.0 / 3.0, 1.0 / 2.0,
     0.5, -0.5, -0.5,   0.0, -1.0,  0.0,  3.0 / 3.0, 1.0 / 2.0,
     0.5, -0.5,  0.5,   0.0, -1.0,  0.0,  3.0 / 3.0, 0.0 / 2.0,
     0.5, -0.5,  0.5,   0.0, -1.0,  0.0,  3.0 / 3.0, 0.0 / 2.0,
    -0.5, -0.5,  0.5,   0.0, -1.0,  0.0,  2.0 / 3.0, 0.0 / 2.0,
    -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,  2.0 / 3.0, 1.0 / 2.0,

    -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,  0.0 / 3.0, 1.0 / 2.0,
     0.5,  0.5, -0.5,   0.0,  1.0,  0.0,  1.0 / 3.0, 1.0 / 2.0,
     0.5,  0.5,  0.5,   0.0,  1.0,  0.0,  1.0 / 3.0, 2.0 / 2.0,
     0.5,  0.5,  0.5,   0.0,  1.0,  0.0,  1.0 / 3.0, 2.0 / 2.0,
    -0.5,  0.5,  0.5,   0.0,  1.0,  0.0,  0.0 / 3.0, 2.0 / 2.0,
    -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,  0.0 / 3.0, 1.0 / 2.0,
];

pub struct Cube {
    scale: f32,
    value: Option<i8>,
    vbo: Option<WebGlBuffer>,
    transform: Isometry3<f32>,
    texture: Option<WebGlTexture>,
    physics: Physics,
    rigid_body_handle: RigidBodyHandle,
}

impl Cube {
    pub fn new(physics: Physics) -> Self {
        let scale = 0.03;

        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![0.0, 0.5, 0.0])
            .linvel(RandomVelocity(5.0, 5.0).get())
            .angvel(RandomAngVel(6.28).get())
            .build();
        let collider = ColliderBuilder::cuboid(scale, scale, scale).restitution(0.7).build();
        let rigid_body_handle = physics.add_rigid_body(rigid_body);
        physics.add_collider_with_parent(collider, rigid_body_handle);

        Self {
            scale: scale * 2.0,
            value: None,
            vbo: None,
            transform: Isometry3::identity(),
            texture: None,
            physics,
            rigid_body_handle,
        }
    }

    pub fn value(&self) -> Option<i8> {
        self.value
    }

    fn transform(&self) -> Isometry3<Real> {
        self.physics.get_rigid_body(self.rigid_body_handle).position().clone()
    }

    pub fn setup_texture(&mut self, gl: &WebGl2RenderingContext) {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        let level = 0;
        let internal_format = WebGl2RenderingContext::RGBA as i32;
        let width = 1;
        let height = 1;
        let border = 0;
        let src_format = WebGl2RenderingContext::RGBA;
        let src_type = WebGl2RenderingContext::UNSIGNED_BYTE;
        let pixel = [0, 0, 255, 255]; // opaque blue
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            level,
            internal_format,
            width,
            height,
            border,
            src_format,
            src_type,
            Some(&pixel),
        ).unwrap();

        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );

        let img = Rc::new(HtmlImageElement::new().unwrap());
        {
            let gl = gl.clone();
            let img = img.clone();
            let _img = img.clone();
            let texture = texture.clone();

            let onload = Closure::wrap(Box::new(move || {
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

                gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                    WebGl2RenderingContext::TEXTURE_2D,
                    level,
                    internal_format,
                    src_format,
                    src_type,
                    &_img,
                )
                .unwrap();

                gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
            }) as Box<dyn FnMut()>);

            img.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
        }
        img.set_src("assets/tex-die.png");

        self.texture = Some(texture);
    }
}

impl Model for Cube {
    fn setup(&mut self, gl: &WebGl2RenderingContext) {
        let buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&CUBE_VERTICES);

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 8 * 4, 0 * 4);
        gl.vertex_attrib_pointer_with_i32(1, 3, WebGl2RenderingContext::FLOAT, false, 8 * 4, 3 * 4);
        gl.vertex_attrib_pointer_with_i32(2, 2, WebGl2RenderingContext::FLOAT, false, 8 * 4, 6 * 4);
        gl.enable_vertex_attrib_array(0);
        gl.enable_vertex_attrib_array(1);
        gl.enable_vertex_attrib_array(2);

        self.setup_texture(gl);

        self.vbo = Some(buffer);
    }

    fn draw(&self, gl: &WebGl2RenderingContext) {
        if let Some(buffer) = &self.vbo {
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
            gl.draw_arrays(
                WebGl2RenderingContext::TRIANGLES,
                0,
                (CUBE_VERTICES.len() / 8) as i32,
            );
        }
    }

    fn update(&mut self, time: f32) {
        self.value = self.physics.value(self.rigid_body_handle);
    }

    fn get_matrix(&self) -> Matrix4<f32> {
        Similarity::from_isometry(self.transform(), self.scale).to_homogeneous()
    }
}

impl Drop for Cube {
    fn drop(&mut self) {
        self.physics.remove_rigid_body(self.rigid_body_handle);
    }
}
