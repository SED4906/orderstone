use img_rs::Image;
use orderstone::{clear, geometry::{Mesh, Vertex}, shader::{Shader, VertexAttribute}, texture::Texture, Application};
use sdl2::{event::Event, keyboard::Keycode};

fn main() {
    Application::run("Window", 800, 600, |mut event_pump, mut canvas| {
        let vertices = vec![
            Vertex{pos: [-0.5,-0.5,0.0], tex_coords: [0.0,0.0]},
            Vertex{pos: [0.5,-0.5,0.0], tex_coords: [1.0,0.0]},
            Vertex{pos: [0.0,0.5,0.0], tex_coords: [0.5,1.0]},
        ];

        let indices = vec![ 0, 1, 2 ];

        let shader = Shader::new(include_str!("vertex.glsl"), include_str!("fragment.glsl"), vec![
            VertexAttribute {attribute_type: gl::FLOAT, normalized: false, size: 3},
            VertexAttribute {attribute_type: gl::FLOAT, normalized: false, size: 2},
        ]);
        let texture = Texture::new(include_bytes!("cheesesticks.png"));
        let mesh = Mesh::new(vertices, indices, shader, texture);

        'running: loop {
            clear(0.8, 0.2, 0.5, 1.0);
            mesh.draw();
            canvas.present();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running Ok(())
                    },
                    _ => {}
                }
            }
            ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }).unwrap();
}