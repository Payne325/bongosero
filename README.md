# Bongosero

A space-invaders style game with a twist

I began developing this game as a way of pracising developing with the Rust Programming Language and as part of a personal effort to be a bit more creative. The initial plan is to develop a simple game in order to focus on developing fun game play and to not overwhelm myself with making too much art and music for a short solo project.
I'm also using this as a topic for discussion at the Cardiff Rust/C++ group bimonthly meetups, with the aim of giving a presentation of the type of projects currently possible with Rust.

The game is controlled with a keyboard, but the end goal is to control the game using the official Nintendo Donkey Konga Bongo controller. Partially because I think its funny, but also because the controllers are so satisfying to use that they lend themselves well for fun gameplay. I also aim to encorporate face/head tracking via a webcam to move the player left and right.

In case you were wondering, someone who plays the bongos is called a bongosero -at least that's what google told me ;)-

I some of the pixel art myself using <a href="https://www.piskelapp.com/">Piskel</a>.

## Dependencies
Game can be built with cargo, this will handle all dependencies except libusb and opencv.
For Linux distros it is likely you already have libusb installed.
For Windows you may have to install it <a href="https://github.com/libusb/libusb/wiki/Windows">manually</a>.

For opencv installation instructions, check the opencv rust crate 
