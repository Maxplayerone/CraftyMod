use gl::types::*;
use std::os::raw::c_void;

#[derive(PartialEq, Eq)]
pub enum VertexArrayConfiguration {
    XyzAndTexCoords,
    XyAndColour,
}

pub struct VertexArray {
    pub id: GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        Self { id }
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }

    pub fn setup_vao(&self, configuration: VertexArrayConfiguration) {
        unsafe {
            if configuration == VertexArrayConfiguration::XyzAndTexCoords {
                self.bind();
                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (5 * std::mem::size_of::<GLfloat>()) as GLint,
                    (0 * std::mem::size_of::<GLfloat>()) as *const c_void,
                );
                gl::EnableVertexAttribArray(0);

                gl::VertexAttribPointer(
                    1,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (5 * std::mem::size_of::<GLfloat>()) as GLint,
                    (3 * std::mem::size_of::<GLfloat>()) as *const c_void,
                );
                gl::EnableVertexAttribArray(1);
            } else if configuration == VertexArrayConfiguration::XyAndColour {
                self.bind();
                gl::VertexAttribPointer(
                    0,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (5 * std::mem::size_of::<GLfloat>()) as GLint,
                    (0 * std::mem::size_of::<GLfloat>()) as *const c_void,
                );
                gl::EnableVertexAttribArray(0);

                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (5 * std::mem::size_of::<GLfloat>()) as GLint,
                    (2 * std::mem::size_of::<GLfloat>()) as *const c_void,
                );
                gl::EnableVertexAttribArray(1);
            }
        }
    }
}
