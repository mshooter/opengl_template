use gl;
use std;
use std::ffi::{CString, CStr};

pub struct Program {
    id: gl::types::GLuint
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe {gl::CreateProgram()};
        for shader in shaders {
            unsafe {gl::AttachShader(program_id, shader.id());}
        }
        unsafe {gl::LinkProgram(program_id);}
        // error handling here
        let mut success: gl::types::GLint = 1; 
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success); 
        }
        
        if success == 0 {
            // return error to buffer -> requires length
            let mut len: gl::types::GLint = 0; 
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            // OPENGL LOG ERROR to our ERROR value 
            unsafe {
                gl::GetProgramInfoLog(
                    program_id, 
                    len, 
                    std::ptr::null_mut(), 
                    error.as_ptr() as *mut gl::types::GLchar);
            }

            // CString -> Rust String
            return Err(error.to_string_lossy().into_owned());
        }

        // need to detach shader before deleting it 
        for shader in shaders {
            unsafe {gl::DetachShader(program_id, shader.id());}
        }

        Ok(Program {id:program_id})
    }
    pub fn id(&self) -> gl::types::GLuint {self.id}
    pub fn set_used(&self) {
        unsafe { gl::UseProgram(self.id); }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe{ gl::DeleteProgram(self.id); }
    }
}


// return struct named shader that wraps the id
pub struct Shader{
    id: gl::types::GLuint
}

impl Shader{
    // acts as a constructor
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?; 
        Ok(Shader {id})
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {self.id}

}

// need to clean up the "shader type leaks shader id"
impl Drop for Shader {
    fn drop(&mut self){
        unsafe{
            gl::DeleteShader(self.id);
        }
    }
}

// Return: Result<OK, FAIL>
// kind = VERTEX or FRAGMENT
fn shader_from_source(source: &std::ffi::CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe {gl::CreateShader(kind)};
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }
    // make an error message 
    let mut success: gl::types::GLint = 1; 
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success); 
    }
    
    if success == 0 {
        // return error to buffer -> requires length
        let mut len: gl::types::GLint = 0; 
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        // OPENGL LOG ERROR to our ERROR value 
        unsafe {
            gl::GetShaderInfoLog(
                id, 
                len, 
                std::ptr::null_mut(), 
                error.as_ptr() as *mut gl::types::GLchar);
        }

        // CString -> Rust String
        return Err(error.to_string_lossy().into_owned());
    }

    Ok (id)
}

// helper function to create new empty CString
fn create_whitespace_cstring_with_len(len: usize) -> CString{
        // allocate buffer of correct size 
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1); 
        // fill it with len spaces
        // ascii space byte
        buffer.extend([b' '].iter().cycle().take(len as usize));
        // buffer -> string 
        unsafe {CString::from_vec_unchecked(buffer)}
}

