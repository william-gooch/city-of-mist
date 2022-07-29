use super::model::Model;
use super::physics::Physics;
use super::shader::Shader;
use super::texture::Texture;
use crate::utils::*;
use rand::Rng;
use rapier3d::na::*;
use rapier3d::prelude::*;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlBuffer, WebGlTexture};

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

#[derive(Clone, Default)]
pub struct CubeBuilder {
    transform: Option<Isometry3<f32>>,
    linvel: Option<Vector3<f32>>,
    angvel: Option<Vector3<f32>>,
    color_primary: Option<Vector4<f32>>,
    color_secondary: Option<Vector4<f32>>,
    texture: Option<Texture>,
}

impl CubeBuilder {
    pub fn build(self, physics: Physics) -> Result<Cube, &'static str> {
        Ok(Cube::new(
            physics,
            self.texture.ok_or("No texture specified")?,
            self.transform.unwrap_or_else(|| Isometry3::identity()),
            self.linvel.unwrap_or_else(|| vector!(0.0, 0.0, 0.0)),
            self.angvel.unwrap_or_else(|| vector!(0.0, 0.0, 0.0)),
            self.color_primary
                .unwrap_or_else(|| vector!(1.0, 0.0, 0.0, 0.0)),
            self.color_secondary
                .unwrap_or_else(|| vector!(0.0, 0.0, 0.0, 0.0)),
        ))
    }

    pub fn texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn random_transform(mut self, rng: &mut impl Rng) -> Self {
        self.transform =
            Some(RandomIsometry(RandomPosition(-0.1, 0.1, 0.5, 0.5, -0.1, 0.1)).get(rng));
        self
    }

    pub fn random_linvel(mut self, rng: &mut impl Rng) -> Self {
        self.linvel = Some(RandomVelocity(5.0, 5.0).get(rng));
        self
    }

    pub fn random_angvel(mut self, rng: &mut impl Rng) -> Self {
        self.angvel = Some(RandomAngVel(6.28).get(rng));
        self
    }

    pub fn color_primary(mut self, color_primary: Vector4<f32>) -> Self {
        self.color_primary = Some(color_primary);
        self
    }

    pub fn color_secondary(mut self, color_secondary: Vector4<f32>) -> Self {
        self.color_secondary = Some(color_secondary);
        self
    }
}

pub struct Cube {
    scale: f32,
    color_primary: Vector4<f32>,
    color_secondary: Vector4<f32>,
    value: Option<i8>,
    vbo: Option<WebGlBuffer>,
    transform: Isometry3<f32>,
    texture: Texture,
    physics: Physics,
    rigid_body_handle: RigidBodyHandle,
}

impl Cube {
    pub fn new(
        physics: Physics,
        texture: Texture,
        transform: Isometry3<f32>,
        linvel: Vector3<f32>,
        angvel: Vector3<f32>,
        color_primary: Vector4<f32>,
        color_secondary: Vector4<f32>,
    ) -> Self {
        let scale = 0.03;

        let rigid_body = RigidBodyBuilder::new_dynamic()
            .position(transform)
            .linvel(linvel)
            .angvel(angvel)
            .build();
        let collider = ColliderBuilder::cuboid(scale, scale, scale)
            .restitution(0.7)
            .build();
        let rigid_body_handle = physics.add_rigid_body(rigid_body);
        physics.add_collider_with_parent(collider, rigid_body_handle);

        Self {
            scale: scale * 2.0,
            color_primary,
            color_secondary,
            value: None,
            vbo: None,
            transform,
            texture,
            physics,
            rigid_body_handle,
        }
    }

    pub fn value(&self) -> Option<i8> {
        self.value
    }

    fn transform(&self) -> Isometry3<Real> {
        self.physics
            .get_rigid_body(self.rigid_body_handle)
            .position()
            .clone()
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

    fn set_uniforms(&self, shader: &Shader) {
        shader.set_vec4("uColorPrimary", &self.color_primary);
        shader.set_vec4("uColorSecondary", &self.color_secondary);
    }
}

impl Drop for Cube {
    fn drop(&mut self) {
        self.physics.remove_rigid_body(self.rigid_body_handle);
    }
}
