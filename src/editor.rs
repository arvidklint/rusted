use std::io::stdout;
use std::time::Duration;

use crossterm::{
    event::{poll, read, Event, KeyCode},
    execute, terminal,
};

use crate::renderer::Renderer;
use crate::window::Window;

pub struct Editor {
    quit: bool,
    window: Window,
}

impl Editor {
    pub fn new() -> Self {
        let mut window = Window::new();
        window.set_active();

        Self {
            quit: false,
            window,
        }
    }

    fn run(&mut self) {
        let mut renderer = Renderer::new();

        self.update(&mut renderer);

        while !self.quit {
            self.process_input();
            self.update(&mut renderer);
        }

        Editor::exit();
    }

    pub fn start(&mut self) {
        terminal::enable_raw_mode().expect("Could not enter terminal raw mode");
        execute!(stdout(), terminal::EnterAlternateScreen)
            .expect("Could not enter alternate terminal screen");

        self.run();
    }

    fn exit() {
        execute!(stdout(), terminal::LeaveAlternateScreen)
            .expect("Could not leave alternate terminal screen");
        terminal::disable_raw_mode().expect("Could not disable terminal raw mode");
    }

    fn update(&mut self, renderer: &mut Renderer) {
        self.window.render(renderer);
    }

    fn read_event() -> Event {
        loop {
            if let Ok(_) = poll(Duration::from_millis(16)) {
                if let Ok(event) = read() {
                    return event;
                }
            }
        }
    }

    fn process_input(&mut self) {
        match Editor::read_event() {
            Event::Key(event) => match event.code {
                KeyCode::Esc => self.quit = true,
                key_code => self.window.process_input(key_code),
            },
            _ => (),
        }
    }
}
