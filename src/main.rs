use minifb::{Key,Window,WindowOptions};


const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer = vec![0;WIDTH*HEIGHT];
    let mut window = Window::new("Test - ESC to exit",
    WIDTH,
    HEIGHT,
    WindowOptions::default()).unwrap_or_else(|e| panic!("{}",e));
    while window.is_open() & !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 255;
        }
        window.update_with_buffer(&buffer).unwrap();
    }
}
