use std::io;
use ratatui::{
    DefaultTerminal, 
    Frame, 
    layout::Constraint, 
    prelude::*,
    widgets::*,
    text::Line, 
};
use crossterm::event::*;
use crossterm::event;

fn main() -> io::Result<()> {

    let mut terminal = ratatui::init();
    
    let mut app = App { exit: false};
    
    let result = app.run(&mut terminal);

    ratatui::restore();
    return result;
}

pub struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>{
        while !self.exit {
            terminal.draw(|frame| self.draw(frame));
        }

        Ok(())
    } 

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
    
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // Render a title on top of the layout
        //Line::from("Process overview").bold().render(area, buf);
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![
                Constraint::Percentage(80),
                Constraint::Percentage(20)])
            .split(area);

        Paragraph::new("outer 0")
            .block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL))
            .render(outer_layout[0], buf);
        
        Paragraph::new("outer 1")
            .block(Block::new().bold().fg(Color::Green).borders(Borders::ALL))
            .render(outer_layout[1], buf);
    }
}