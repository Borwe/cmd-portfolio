use std::{io, str::FromStr, thread::sleep, time::Duration};

use crossterm::{event::{self, KeyCode, KeyEventKind}, style::Stylize};
use rand::Rng;
use ratatui::{
     layout::Rect, text::Line, widgets::{Block, Paragraph, Widget}, Frame
};

struct HelloPortfolioWidget {
    text: &'static str,
    yoffset: u16,
    xoffset: u16,
}

impl HelloPortfolioWidget {
    fn new() -> Self {
        Self {
            text: "This is my Portfolio Project in development🪓🪓 (q to quit)",
            yoffset: 4,
            xoffset: 4,
        }
    }
}

impl Widget for &HelloPortfolioWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let line = Line::from(self.text);
        let block = Block::bordered()
            .title("🚧🚧🚧🚧Under construction🚧🚧🚧🚧")
            .title_alignment(ratatui::layout::Alignment::Center)
            .border_type(ratatui::widgets::BorderType::Thick);
        block.render(area, buf);
        let new_area = Rect::new(self.xoffset, self.yoffset, line.width() as u16, 2);
        line.render(new_area, buf);
    }
}

struct App {
    exit: bool,
    hello_widget: HelloPortfolioWidget,
}

impl App {
    fn new() -> Self {
        App {
            exit: false,
            hello_widget: HelloPortfolioWidget::new(),
        }
    }

    fn run<B>(&mut self, mut term: ratatui::Terminal<B>) -> Result<(), Box<dyn std::error::Error>>
    where
        B: ratatui::backend::Backend,
    {
        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            sleep(Duration::from_millis(80));
        }
        Ok(())
    }

    fn handle_events(&mut self)-> io::Result<()> {
        match event::poll(Duration::from_millis(9))?{
            true => match event::read()?{
                event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    if key_event.code == KeyCode::Char('q') {
                        self.exit = true
                    }
                    Ok(())
                },
                _ => Ok(())
            },
            false => Ok(()),
        }
    }

    fn update_offset_from_center(&mut self, area: Rect){
        let mut rng = rand::thread_rng();
        self.hello_widget.yoffset = {
            let val = rng.gen_range(-1..=1);
            if val < 0 {
                self.hello_widget.yoffset.overflowing_sub(1).0
            }else{
                self.hello_widget.yoffset.overflowing_add(val as u16).0
            }
        };
        self.hello_widget.xoffset = {
            let val = rng.gen_range(-1..=1);
            if val < 0 {
                self.hello_widget.xoffset.overflowing_sub(1).0
            }else{
                self.hello_widget.xoffset.overflowing_add(val as u16).0
            }
        };

        if self.hello_widget.yoffset <= 1 {
            self.hello_widget.yoffset = 1
        }

        if self.hello_widget.xoffset <= 1 {
            self.hello_widget.xoffset = 1
        }

        let max_x = area.width - self.hello_widget.text.len() as u16 ;
        if self.hello_widget.xoffset >= max_x -2 {
            self.hello_widget.xoffset = max_x - 2 
        }
        if self.hello_widget.yoffset >= area.height -2  {
            self.hello_widget.yoffset = area.height -2;
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.update_offset_from_center(frame.area());
        frame.render_widget(&self.hello_widget, frame.area());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    let mut app = App::new();
    app.run(terminal)?;
    ratatui::restore();
    Ok(())
}
