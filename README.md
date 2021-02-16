# Arecibo

Arecibo is an application for determining which validators have not yet come online in a stuck cosmos network.

In a network that is just starting out, it expects a `genesis.json` file in the current directory. 

A run command looks like this

``` bash
cargo run -- start  tcp://34.71.170.158:26657
```


## Getting Started

This application is authored using [Abscissa], a Rust application framework.

For more information, see:

[Documentation]

[Abscissa]: https://github.com/iqlusioninc/abscissa
[Documentation]: https://docs.rs/abscissa_core/
