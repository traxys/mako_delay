# mako_delay
Making mako notifications disapear after some time

## Usage

You can rust `mako_delay -t <TIMEOUT>` to have notifications disappear after `TIMEOUT` seconds, defaults to 5.

## Compiling

This project uses rust, to compile it you need at least a version 1.39 of the rust compiler. Installing the rust compiler is described [here](https://www.rust-lang.org/tools/install)
When you have a rust compiler you can just run `cargo build --release` to build from source, and `cargo install --path .` to install it. You can also do `cargo install --git https://github.com/traxys/mako_delay` directly
