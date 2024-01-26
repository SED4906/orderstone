use std::error::Error;
pub mod geometry;
pub mod shader;
pub mod buffers;
pub mod texture;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub fn clear(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub struct Application();

impl Application {
    pub fn run(title: &str, width: u32, height: u32, runner: impl Fn(sdl2::EventPump, sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), Box<dyn Error>>) -> Result<(), Box<dyn Error>> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(title, width, height)
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .build()
            .unwrap();
        let canvas = window.into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();
    
        // initialization
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    
        // sdl::render creates a context for you, if you use a Canvas you need to use it.
        canvas.window().gl_set_context_to_current().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        runner(event_pump, canvas)
    }
}