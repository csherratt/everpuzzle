![screenshot](https://github.com/Skytrias/everpuzzle/blob/master/everpuzzle-04.gif "preview")

# Everpuzzle
**Tetris Attack/Pokemon Puzzle esque game written in Rust with Ametyhst**

Talk / Code Walkthrough on this project https://www.youtube.com/watch?v=P_9A7P0uNpY

## Goal
Build a clone similar to Tetris Attack and Pokémon Puzzle. This game needs to abide by the original rules of the game, so for example hit right frame times to feel exactly like the old games.

## Rust
I've tried many languages and rust's vision of safe multithreading is something I believe is important for games. So the goal is to also make this game run as fast as possible. If you have any ideas on how to improve the speed of the game, hit me up. 

This is also a way for me to learn Rust more and to help it grow.

## [Amethyst Game Engine](https://github.com/amethyst/amethyst)
I've tried out most game engines there are right now for rust and all of them are heavily in development. Some are unmaintained or just don't have any visions for the future. As well it's important to me that the engine I'm using tries to achieve multithreading. And thats what Amethyst tries! So even if it's harder to make games with it, I'll use it. 

## How to Build
If you're new to Rust: [Download Rust](https://www.rust-lang.org/en-US/install.html)

1. Clone/Download this repository into a folder
2. Open a Command Line inside the folder and run: cargo run (Downloading all crates will take some time...)
3. If you get errors make sure to update your Rust Version before creating issues here! Run: rustup update

If any steps were unclear or you had any issues, please open an issue. 

## How to Play
|  Action  | Keyboard  | Controller  |
| ------------ | ------------ | ------------ |
|  Move  | WASD / Arrows  |  Analog Arrows  |
| Swap  | X / Y  |  A / B  |
| Raise | C / V  | L / R  |
| Reset | Space  | Select  |
| Menu | Enter | Start |

## About me
*Skytrias #8787 on Discord*

Whenever I try out new game engines/frameworks I start to make a clone of Tetris Attack just to get started. In the past I've done this a lot and I tried many ways to program parts of the logic. The old games are very logic intensive to it takes time until something you get something that works and is expandable.

I've worked on [swap'n'pop](https://github.com/omenking/swap-n-pop) but the project seems to be dead which is sad. But it also had problems imo.

## Contributing
If you are interested in helping out you can take a look at the [issues](https://github.com/Skytrias/rust-attack/issues) and work on anything you'd want. Otherwhise you can contact me on Discord ***Skytrias #8787***.

## Links
[Spread sheet for frame times](https://docs.google.com/spreadsheets/d/1SsVXHad0z7Dbsqfj-UTd4HZSGCslujkbh7vOan61D1g/edit#gid=1601136205) 
[Tetris Attack Discord](https://discordapp.com/invite/CxJwFFX)

## License
Rust-Attack is free and open source software distributed under the terms of both the [MIT License](https://github.com/Skytrias/rust-attack/blob/master/LICENSE).

The rights to the original Tetris Attack sprites belong to Nintendo. In the future I'd like to use new art.
