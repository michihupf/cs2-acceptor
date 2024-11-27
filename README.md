# cs2-acceptor
The cs2-acceptor can automatically accept CS2 matches for you so you can take a toilet break while queuing.

<img src="https://raw.githubusercontent.com/michihupf/cs2-acceptor/refs/heads/master/toilet-cat.jpg" alt="drawing" width="200"/>

## Building from Source
(For Windows users a pre-built binary is available to [download here](https://github.com/michihupf/cs2-acceptor/releases/latest).)

To build from source make sure you have the [Rust](https://www.rust-lang.org/tools/install) ecosystem installed. Then run

```
cargo build --release
```

The binary will be produced in `target/<triple>/release/cs2-acceptor`. If you want to add a icon to the executable, 
the file can be found in [`assets`](https://github.com/michihupf/cs2-acceptor/tree/master/assets) directory.

## Usage

1. Open the executable
2. Tab back into CS2 (the game has to be in view so that the button is detected)
3. Queue up
4. Enjoy your match after you come back from the toilet

The program is set to kill itself when pressing [B]. This is both a failsafe if the program is misbehaving and a convenience to close it when playing.

## Is this VAC-safe?

As the program does not interact with the game and generally is inactive while you play you should be safe from VAC bans.
Although it should not happen, I **do not** take any responsibility for any VAC bans!!

**I STRONGLY RECOMMEND TO KILL THE PROGRAM WHEN INGAME AS IT MIGHT CLICK GREEN AREAS BY ACCIDENT AND *THIS* MIGHT CAUSE FLAGGED GAMEPLAY.**

The [B] key is carefully chosen as the killswitch as it is the default key for the CS2 buy menu.
In case you forget to stop the program it will kill itself when you open the buy menu (pressing the [B] key).

## How it works

First the program looks for the CS2 window. If it isn't found it will wait.
Once the window has been found it will be able to identify which monitor the game runs on.

From there it will capture a screenshot of that screen every second. This screenshot is then analyzed to detect the color of the ACCEPT button.
Don't worry those screenshots are only kept in memory and are not saved to your machine.

If your game is minimized at any point the screengrabbing will be paused until the game is in view again.
When you quit the game `cs2-acceptor` will also quit.

## Alternative ways of starting

### Add as non-steam game
If you want the executable to be runnable directly from Steam you can add it as a non-steam game.

To do so, on the top bar in the Steam client, 

1. click `Games > Add a Non-Steam Game to my Library`.
2. click `Browse`
3. select `cs2-acceptor.exe` (or the relevant executable when not on Windows)

You can now start the program via the Steam Library.

### Add a AutoHotkey script
You can use tools like [AutoHotkey](https://www.autohotkey.com/) to create a hotkey that launches `cs2-acceptor.exe`.

I will not give specific instructions on how to do this.

## Contributing
Feel free to contribute or report any issues.
