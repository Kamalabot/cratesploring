#![allow(warnings)]

use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
// alternate screen, raw model and backend
fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut term = ratatui::init();
    let app_res = run(term);
    ratatui::restore();
    app_res
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greet = Paragraph::new("Hello There. (Press q to exit).".to_owned())
                .blue()
                .on_red();
            frame.render_widget(greet, frame.area());
        });
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
