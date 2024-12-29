# Textris

This is a command-line Tetris clone. I have created it for educational purpose. The development of this game has been recorded and published on YouTube.

As mentioned above, this game is a pure-console (text) based game, and it is not using any graphical game engine. All the characters are UNICODE emojis. Therefore the look and feel can vary greatly based on the selected font used whenever it is started.

The game is cross-platform, I have tested it on Linux and Windows environments (I do not own a Mac).

On Linux terminals it works and looks quite nice, but unfortunately on Windows the game looks a bit wonky due to the fact that Windows doesn't properly support UNICODE with its `Cmd` or `PowerShell` tools. I was able to make it look somewhat acceptable by using `Windows Terminal` and by chaning to `Cascadia Mono` font.

## Building

To build the game, one needs to first install the [Rust toolchain](https://www.rust-lang.org/tools/install).

The navigate to the project's root directory and run `cargo build --release`

Alternatively, I have created a Bash script (called `release.sh`) which you can use to generate two ZIP files containing the Linux and Windows executables, respectively.

## Running

When running the game, make sure that the "assets" directory is directly under the current directory, so that the application can find it.

So one can use the above-mentioned ZIP files, extract them and play the game directly, or if you have the Rust toolchain installed, just run it invoking `cargo run --release`
