mod state;
mod debug_controller;
mod world;
mod phys;

use quicksilver::{
   geom::{Rectangle, Vector},
   graphics::{Color, Image},
   input::Key,
   run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(    
       Settings {
          title: "Bongosero",
          size: Vector::new(800.0, 600.0),
          ..Settings::default()
       },
       app,
      );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()>{
   
   //Initialise background 
   let background = Image::load(&gfx, "background.png").await?;
   let background_region = Rectangle::new(Vector::new(0.0, 0.0), background.size());

   let player_sprite = Image::load(&gfx, "mc_spritesheet.png").await?;
   let mut state = state::State::new().unwrap();

   println!("State initialised...\n");

   loop {
      while let Some(_) = input.next_event().await {}
      //Game logic
      state.update(&input);

      //Draw
      gfx.clear(Color::BLACK);
      gfx.draw_image(&background, background_region);

      let player_region = Rectangle::new(state.position_data(), player_sprite.size());
      gfx.draw_image(&player_sprite, player_region);

      let _res = gfx.present(&window);

      //Handle exit
      if input.key_down(Key::Escape){
         println!("Goodbye!");
         break;
      }
   }

   Ok(())
}
