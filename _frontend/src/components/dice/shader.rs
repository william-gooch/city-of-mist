use super::texture::Texture;
use rapier3d::{na::*, prelude::*};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub struct ShaderBuilder {
    vert_source: Option<String>,
    frag_source: Option<String>,
}

pub struct Shader {
    context: WebGl2RenderingContext,
    program: WebGlProgram,
}

impl ShaderBuilder {
    pub fn new() -> Self {
        Self {
            vert_source: None,
            frag_source: None,
        }
    }

    pub fn vert(mut self, vert_source: impl Into<String>) -> Self {
        self.vert_source = Some(vert_source.into());
        self
    }

    pub fn frag(mut self, frag_source: impl Into<String>) -> Self {
        self.frag_source = Some(frag_source.into());
        self
    }

    pub fn build(self, context: &WebGl2RenderingContext) -> Result<Shader, String> {
        let vert_shader = Shader::compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            &self
                .vert_source
                .ok_or_else(|| String::from("Vertex shader source not set."))?,
        )?;

        let frag_shader = Shader::compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            &self
                .frag_source
                .ok_or_else(|| String::from("Fragment shader source not set."))?,
        )?;

        let program = Shader::link_program(&context, &vert_shader, &frag_shader)?;

        Ok(Shader {
            program,
            context: context.clone(),
        })
    }
}

impl Shader {
    pub fn activate(&self) {
        self.context.use_program(Some(&self.program));
    }

    pub fn set_vec4(&self, name: &str, vector: &Vector4<Real>) {
        let loc = self.context.get_uniform_location(&self.program, name);
        self.context
            .uniform4fv_with_f32_array(loc.as_ref(), vector.as_slice());
    }

    pub fn set_mat4(&self, name: &str, matrix: &Matrix4<Real>) {
        let loc = self.context.get_uniform_location(&self.program, name);
        self.context
            .uniform_matrix4fv_with_f32_array(loc.as_ref(), false, matrix.as_slice());
    }

    pub fn set_texture(&self, name: &str, texture: &Texture) {
        self.context
            .active_texture(WebGl2RenderingContext::TEXTURE0);
        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture.texture()));

        let loc = self.context.get_uniform_location(&self.program, name);
        self.context.uniform1i(loc.as_ref(), 0);
    }

    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(
        context: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create program object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program")))
        }
    }
}
