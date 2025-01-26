use crossterm::style::Color;
use ratatui::widgets::{Block, Widget};
use ratatui::prelude::*;


pub struct KenyanFlag;


impl KenyanFlag {
    pub fn new()-> Self{
        Self{}
    }

    fn calc_black(&self, area: &Rect)-> Rect{
        let mut height: f32 = area.height as f32;
        height = height * 0.25;
        Rect::new(area.x, area.y, area.width, height as u16 + 1)
    }
    fn calc_white1(&self, area: &Rect)-> Rect{
        let black = self.calc_black(area);
        let mut height: f32 = area.height as f32;
        height = height * 0.125;
        Rect::new(area.x, black.y + black.height,
            area.width, height as u16)
    }
    fn calc_red(&self, area: &Rect)-> Rect{
        let white_top = self.calc_white1(area);
        let mut height: f32 = area.height as f32;
        height = height * 0.25;
        Rect::new(area.x, white_top.y + white_top.height,
            area.width, height as u16 + 1 )
    }
    fn calc_white2(&self, area: &Rect)-> Rect{
        let red = self.calc_red(area);
        let mut height: f32 = area.height as f32;
        height = height * 0.125;
        Rect::new(area.x, red.y + red.height ,
            area.width, height as u16)
    }
    fn calc_green(&self, area: &Rect)-> Rect{
        let white_bottom = self.calc_white2(area);
        let mut height: f32 = area.height as f32;
        height = height * 0.25;
        Rect::new(area.x, white_bottom.y + white_bottom.height ,
            area.width, height as u16 + 1)
    }
}

impl Widget for &KenyanFlag{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let black_area = self.calc_black(&area);
        let white1_area = self.calc_white1(&area);
        let red_area = self.calc_red(&area);
        let white2_area = self.calc_white2(&area);
        let green_area = self.calc_green(&area);

        Block::new().bg(Color::Black).render(black_area, buf);
        Block::new().bg(Color::White).render(white1_area, buf);
        Block::new().bg(Color::Red).render(red_area, buf);
        Block::new().bg(Color::White).render(white2_area, buf);
        Block::new().bg(Color::Green).render(green_area, buf);
    }
}
