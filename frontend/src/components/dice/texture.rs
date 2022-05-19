use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};

#[derive(Clone)]
pub struct Texture {
    texture: WebGlTexture,
}

impl Texture {
    pub fn new(gl: &WebGl2RenderingContext, src: &str) -> Self {
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
        )
        .unwrap();

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
        img.set_src(src);

        Self { texture }
    }

    pub fn texture(&self) -> &WebGlTexture {
        &self.texture
    }
}
