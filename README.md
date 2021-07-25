# Bongosero

A proof of concept for a space-invaders style game with face tracking for character movement and support for Gamecube Donkey Konga Bongos to fire the gun.

I began developing this as a way of pracising developing with the Rust Programming Language. As such there could be neater, more idiomatic ways of writing the code found here. If you have any suggestions please feel free to raise an issue.

The game can be controlled with a keyboard for debugging purposes, but the intended use is to control the game using the official Nintendo Gamecube adapter for Wii U, official Nintendo Donkey Konga Bongo controller and a webcam.

I created some of the pixel art myself using <a href="https://www.piskelapp.com/">Piskel</a>.

In case you were wondering, someone who plays the bongos is called a bongosero (well that's what google told me anyway ;) ).

## Dependencies
The game is expected to be built with cargo, but there are a couple of additional dependencies that need to be handled.

### Face tracking for player movement
Face tracking is handled by the [`f-trak`](https://github.com/Payne325/f-trak) crate, which relies on the [`opencv-rust`](https://github.com/twistedfall/opencv-rust) crate. Please follow the set up procedure for `opencv-rust` in its documentation.

### Handling the Gamecube controller input
Gamecube Controller input is handled by the [`gc-adapter`](https://github.com/jam1garner/gc-adapter) crate, which creates a libusb interface to the official Nintendo Gamecube Adapter for Wii U. I believe it should also work with the Mayflash adapter but this is untested.

For Linux based distros, you shouldn't have any problems.
For Windows you may have to install the necessary driver for `gc-adapter` to detect any devices. The setup documentation for the [`Dolphin Emulator`](https://dolphin-emu.org/docs/guides/how-use-official-gc-controller-adapter-wii-u/#Windows) provides instructions on how to install this.  

## Building

`Bongosero` has two optional features which can be specified at compile time.

`debug` - Prints debug information to standard output while game is executing.
`keyboard` - compiles keyboard controls instead of bongo and face tracking.

```
cargo run --feature "keyboard debug"
```
