//use crate::controller;
use crate::debug_controller;
use quicksilver as qs;
use quicksilver::geom::Shape;
use quicksilver::graphics as gfx;
use quicksilver::graphics::{Background::Img, Image};
use quicksilver::lifecycle::Asset;


pub struct State {
    //m_bongo : controller::Controller,
    m_background : Asset<Image>,
    m_controller : debug_controller::Debug_Controller
}

impl qs::lifecycle::State for State {
    fn new() -> qs::Result<Self> {

        //let bongo = controller::Controller::new();
        let background = Asset::new(Image::load("background.png"));
        let controller = debug_controller::Debug_Controller::new();

        Ok(
            Self {
                //m_bongo : bongo,
                m_background : background,
                m_controller : controller
            }
        )
    }

    fn update(&mut self, window: &mut qs::lifecycle::Window) -> qs::Result<()> {
        
      self.m_controller.poll(window);
      self.m_controller.print();
        //self.m_bongo.poll();
        //self.m_bongo.print();

      Ok(())
    }

    fn draw(&mut self, window: &mut qs::lifecycle::Window) -> qs::Result<()> {
        
        let mut result = window.clear(gfx::Color::BLACK);

        result = self.m_background
        .execute(|image| {
                window.draw(
                    &image.area().with_center((400, 300)), 
                    Img(&image));

                    Ok(())
            }
        );

        Ok(())
    }
}