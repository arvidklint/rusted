use crossterm::event::KeyCode;

use crate::buffer::Buffer;
use crate::renderer::Renderer;

pub struct Window {
    active: bool,
    buffer: Buffer,
}

impl Window {
    pub fn new() -> Self {
        let buffer = Buffer::new("This is a test buffer\nwith multiple lines\n");
        Self {
            active: false,
            buffer,
        }
    }

    pub fn process_input(&mut self, key_code: KeyCode) {
        match key_code {
            key_code => self.buffer.process_input(key_code),
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        self.buffer.render(renderer);

        renderer.flush();
    }

    pub fn set_active(&mut self) {
        self.active = true;
    }
}
