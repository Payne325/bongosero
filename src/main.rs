mod game;
mod input_device;
mod world;
mod phys;

#[cfg(feature = "keyboard")]
mod keyboard_weapon;
#[cfg(feature = "keyboard")]
mod keyboard_movement;

#[cfg(not(feature = "keyboard"))]
mod bongosero_movement;
#[cfg(not(feature = "keyboard"))]
mod bongosero_weapon;

use quicksilver::{
   geom::Vector,
   graphics::Image,
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
   // This function serves as the control loop, on exit the game ends.
   
   //Load images 
   let background = Image::load(&gfx, "background.png").await?;
   let player_sprite = Image::load(&gfx, "mc_spritesheet.png").await?;
   let bullet_sprite = Image::load(&gfx, "bullet.png").await?;
   let start_msg = Image::load(&gfx, "start_msg.png").await?;
   let enemy_sprite = Image::load(&gfx, "enemy.png").await?;

   //Construct object to handle main game functionality.
   let mut game = game::Game::new(background, player_sprite, bullet_sprite, start_msg, enemy_sprite).unwrap();

   println!("Game manager initialised...\n");

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
