use std::path::Path;

extern crate image;
use image::EncodableLayout;

pub struct Texture{
    tex_id: u32,
}

impl Drop for Texture{
    fn drop(&mut self){
        unsafe{
            gl::DeleteTextures(1, [self.tex_id].as_ptr());
        }
    }
}

impl Texture{
    pub unsafe fn new(filepath: &Path) -> Self{
        let mut id: u32 = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

         
        let img = image::open(filepath).unwrap().into_rgba8();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGBA as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0, 
                       gl::RGBA,
                       gl::UNSIGNED_BYTE,
                       img.as_bytes().as_ptr() as *const _
                       );                  
        gl::GenerateMipmap(gl::TEXTURE_2D);

        Self {tex_id: id}
    }

    pub fn bind(&self){
        unsafe{
        gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
    }
    }
}