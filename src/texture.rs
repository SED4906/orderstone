use std::ffi::c_void;

use img_rs::Image;

pub struct Texture {
    pub texture: u32
}

impl Texture {
    pub fn new(data: &[u8]) -> Self {
        let mut texture = 0;
        unsafe{img_rs::sys::stbi_set_flip_vertically_on_load(1);}
        let image = Image::from_bytes(data, 3).unwrap();
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, image.width() as i32, image.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, image.bytes().as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Self { texture }
    }

    pub fn bind(&self) {
        unsafe{
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }
}