# sudoku-www

A basic Sudoku solver using [Rust](https://www.rust-lang.org/) and [Seed](https://github.com/David-OConnor/seed). It compiles to WebAssembly and runs entirely client-side. Seed uses a minimal JavaScript-wrapper to call the WebAssembly.

Example is currently running [here](http://skrimstad.net:8000).

## Algorithm

This solution uses a simple backtracking algorithm to solve the sudoku. It simply attempts all possible solutions for the board, it backtracks whenever it detects that this cannot be the solution. E.g. when we've placed multiple identical numbers in the current row, column or box.

It is not efficient at all. I believe the worst case time complexity is O(N^(N^2)) for an NxN Sudoku.

## Dependencies

* [Rust](https://www.rust-lang.org/tools/install)
* Possibly [Docker](https://www.docker.com/) and [Docker Compose](https://docs.docker.com/compose/) if you want to deploy it using Docker.

## Building and running

### Getting dependencies

To get the required dependencies you can run:

`cargo build`

### Building debug

To compile the project with debug flags and generate the required files:

`cargo make all`

### Building release

For deployment it is strongly recommended to build the release version as the size will be much smaller:

`cargo make all_release`

### Running development server

To run a development server you can use this command. The server will be listening on port 8000.

`cargo make serve`

### Running docker server

To run a docker server you can use docker-compose. The server will be listening on port 8000.

`docker-compose up`
