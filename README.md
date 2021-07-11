# Bongosero

A space-invaders style game with a twist

I began developing this game as a way of pracising developing with the Rust Programming Language and as part of a personal effort to be a bit more creative. The initial plan is to develop a simple game in order to focus on developing fun game play and to not overwhelm myself with making too much art and music for a short solo project.
I'm also using this as a topic for discussion at the Cardiff Rust/C++ group bimonthly meetups, with the aim of giving a presentation of the type of projects currently possible with Rust.

The game is controlled with a keyboard, but the end goal is to control the game using the official Nintendo Donkey Konga Bongo controller. Partially because I think its funny, but also because the controllers are so satisfying to use that they lend themselves well for fun gameplay. I also aim to encorporate face/head tracking via a webcam to move the player left and right.

In case you were wondering, someone who plays the bongos is called a bongosero -at least that's what google told me ;)-

I some of the pixel art myself using <a href="https://www.piskelapp.com/">Piskel</a>.

## Dependencies
While the game can be built with cargo, there are a couple of additional dependencies thta need to be handled.

### Face tracking for player movement
Face tracking is handled by the [`f-trak`](https://github.com/Payne325/f-trak) crate, which relies on the [`opencv-rust`](https://github.com/twistedfall/opencv-rust) crate. Please follow the set up procedure for `opencv-rust` in its documentation.

### Handling the Gamecube controller input
Gamecube Controller input is handled bu the [`gc-adapter`](https://github.com/jam1garner/gc-adapter) crate, which creates a libusb interface to the official Nintendo Gamecube Adapter for Wii U. I believe it should also work with mayflash but this is untested.

For Linux based distros, you shouldn't have any problems.
For Windows you may have to install the necessary driver for `gc-adapter` to detect any devices. The setup documentation for the [`Dolphin Emulator`](https://dolphin-emu.org/docs/guides/how-use-official-gc-controller-adapter-wii-u/#Windows) provides instructions on how to install this.  