use rapier3d::{prelude::*, na::*};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader};

pub trait Model {
    fn setup(&mut self, gl: &WebGl2RenderingContext);
    fn draw(&self, gl: &WebGl2RenderingContext);
    fn update(&mut self, time: f32);
    fn get_matrix(&self) -> Matrix4<Real>;
}
