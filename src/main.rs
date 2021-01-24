mod game;
mod controller;
mod bongosero_controller;
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
   /*
    * This function serves as the control loop, on exit the game ends.
    * Todo: Add a splash screen and menu for options, camera calibration and highscore data. 
   */

   //Load images 
   let background = Image::load(&gfx, "background.png").await?;
   let player_sprite = Image::load(&gfx, "mc_spritesheet.png").await?;
   let bullet_sprite = Image::load(&gfx, "bullet.png").await?;

   //Construct object to handle main game functionality.
   let mut game = game::Game::new(background, player_sprite, bullet_sprite).unwrap();

   println!("Game manager initialised...\n");
//test test test, i fucked up my hand yesterday at the dump and had to go to a and e. I wrote this sentence to see how well i can type.,,,
   loop {
      //Handle keyboard input
      //Todo: replace this with bongo/webcam controls
      while let Some(_) = input.next_event().await {}
      game.update(&input);

      //Draw
      gfx = game.draw(gfx);
      let _res = gfx.present(&window);

      //Handle exit
      if input.key_down(Key::Escape){
         println!("Goodbye!");
         break;
      }
   }

   Ok(())
}
