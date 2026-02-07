# Textris

This is a command-line Tetris clone. I have created it for educational purpose. The development of this game has been recorded and published on YouTube. For the more nerdy amongst you, here is the [YouTube video](https://www.youtube.com/watch?v=DpJJtJf6sNY) link.

As mentioned above, this game is a pure-console (text) based game, and it is not using any graphical game engine. All the characters are UNICODE emojis. Therefore the look and feel can vary greatly based on the selected font used whenever it is started.

The game is cross-platform, it has been tested it on Linux and Windows environments. Mac support is unknown, as I do not own a Mac device.

## Building the game

To build the game, one needs to first install the [Rust toolchain](https://www.rust-lang.org/tools/install).

### For Linux systems

For Linux, the recommended way is to build the Debian (.deb) package. In order to do that, first install cargo-deb:

```sh
$> cargo install cargo-deb
```

Once you have cargo deb, the command to build the .deb package is:

```sh
$> cargo deb
```

This will construct the debian package under `target/debian`. Then simply deploy install the package on the Linux syustem using the `apt install` command.

### For Windows systems

Simply launch the bash script that generates a ZIP file that contains the Windows version of Textris:

```sh
$> ./release-win.sh
```

This will generate a `textris.zip` file that can be deployed and unzipped on a modern Windows system.

## Installation and running

### In Linux

If you installed the debian package, simply launch in a terminal:

```sh
$> apt install -i <path-to-deb-file>
$> textris
```

### In Windows

Simply unzip and run the program:

```sh
$> unzip textris.zip
$> .\textris.exe
```

## Note

On Linux terminals the game looks quite nice, but unfortunately on Windows systems the game looks a bit wonky by default, due to the fact that Windows doesn't properly support UNICODE with its `Cmd` or `PowerShell` tools.

I was able to make it look somewhat acceptable by using `Windows Terminal` and changing to `Cascadia Mono` font.
