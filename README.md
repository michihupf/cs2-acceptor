# cs2-acceptor

The cs2-acceptor can automatically accept CS2 matches for you so you can take a toilet break while queuing.

<img src="https://raw.githubusercontent.com/michihupf/cs2-acceptor/refs/heads/master/toilet-cat.jpg" alt="drawing" width="200"/>

It works by capturing a screenshot of your primary screen every second to detect the color of the ACCEPT button.
Don't worry those screenshots are only kept in memory and are not saved to your machine.

## Usage

1. Open the executable
2. Tab back into CS2 (the game has to be in view so that the button is detected)
3. Enjoy your Matchmaking game after you come back from the toilet

The program is set to kill itself when pressing [B]. This is both a failsafe if the program is misbehaving and a convenience to close it when playing.

**I STRONGLY RECOMMEND TO KILL THE PROGRAM WHEN INGAME AS IT MIGHT CLICK GREEN AREAS BY ACCIDENT**

## Building from Source
(For Windows users a pre-built binary is available in the [latest release](https://github.com/michihupf/cs2-acceptor/releases/tag/release).)

To build from source make sure you have the [Rust](https://www.rust-lang.org/tools/install) ecosystem installed. Then run

```
cargo build --release
```

The binary will be produced in `target/<triple>/release/cs2-acceptor`.

## Contributing
Feel free to contribute or report any issues.
