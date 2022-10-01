# Rattle

Multiplayer code battles to have fun while learning Rust. 🎮🦀

## Architecture

A [VueJS](https://vuejs.org/) frontend communicates with an [Iron](https://github.com/iron/iron)
backend, originally from the [Rust Playground](https://github.com/integer32llc/rust-playground). [Docker][docker] containers are used to provide the various
compilers and tools as well as to help isolate them. 

## How the game works

The game assumes 2 players are online and ready to play. Following this:

1. A player chooses an exercise and gets matched to the other available player.
1. For each player, a websocket connection is established between their browser and the server.
1. Players are prompted with a fun text giving context to the exercise and have a limited amount of time to submit a solution. The amount of time depends on the 
player mode they're in, for e.g. Blitz Mode, Extended Play Mode and Collaborative Mode. Each exercise is Rust code that doesn't pass a set of tests. Players must fix the code so that it passes the tests. 
1. Once a timer starts, players must submit a solution before the timer runs out. The play whose code passes the tests wins a reputation point.

## Development

### Build the UI

```
cd frontend
npm i
npm run serve
```

### Build and run the server

```
cd ui
cargo run
```

There are some optional configuration parameters which you may set in a 
`.env` file. The server will run with no configuration, but in order 
to load and save gists a GitHub token must be configured.

### Build or download the containers

```
cd compiler
./build.sh # If you want to test changes to the containers
./fetch.sh # If you just want the current playground
```

## Resource limits

### Network

There is no network connection between the compiler container and the
outside world.

### Memory

The amount of memory the compiler and resulting executable use is
limited by the container.

### Execution Time

The total compilation and execution time is limited by the container.

### Disk

This sandbox **does not** provide any disk space limits. It is
suggested to run the server such that the temp directory is
space-limited. One bad actor may fill up this shared space, but it
should be cleaned when that request ends.

## License

Licensed under Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0).