use std::{io, time::Duration};
use ratatui::{
    Terminal,
    crossterm::{
        event::{
            self, 
            DisableMouseCapture, 
            EnableMouseCapture, 
            Event,
        },
        execute,
        terminal::{
            EnterAlternateScreen,
            LeaveAlternateScreen,
            disable_raw_mode,
            enable_raw_mode
        }
    }, 
    prelude::{
        Backend,
        CrosstermBackend
    }
};
use crate::app::App;
use crate::ui::render;
mod app;
mod random;
mod ui;
fn main() {
    if let Err(error) = enable_raw_mode() {
        panic!("error occur: {:?}", error);
    };
    let mut stderr = io::stderr();
    if let Err(error) = execute!(
        stderr,
        EnterAlternateScreen,
        EnableMouseCapture
    ) {
        panic!("error occur: {:?}", error);
    };
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(error) => {
            panic!("error occur: {:?}", error);
        }
    };

    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    if let Err(error) = disable_raw_mode() {
        panic!("error occur: {:?}", error);
    };
    if let Err(error) = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ) {
        panic!("error occur: {:?}", error);
    };
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error>
where io::Error: From<B::Error> {
    loop {
        if let Err(error) = terminal.draw(|frame| {
            render(frame, app);
        }) {
            panic!("error occur: {:?}", error);
        };
        match event::poll(Duration::from_millis(1000 / 60)) {
            Ok(polled) => {
                if polled {
                    if let Event::Key(key) = match event::read() {
                        Ok(event) => {
                            event
                        },
                        Err(error) => {
                            panic!("error occur: {:?}", error);
                        }
                    } {
                        app.input(key.code);
                    }
                }
            }
            Err(error) => {
                panic!("error occur: {:?}", error);
            }
        }
        if app.is_exit {
            break;
        }
    }
    Ok(())
}