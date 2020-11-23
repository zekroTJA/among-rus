# Among (R)Us

`among-rus` *(stylized `Among (R)Us`)* is a little side project of mine to get deeper into the Rust language and eco system. The goal of the project is to re-implement a server application for the game [Among Us](https://innersloth.com/gameAmongUs.php) by [InnerSloth](https://innersloth.com).

The project is based on the [writeup of the network protocol](https://github.com/codyphobe/among-us-protocol) of the game by [clodyphobe](https://github.com/codyphobe).

The code is based on the [async-rs](https://github.com/async-rs/async-std) crate for the UDP networking implementation. To be honest, I have no idea if I am using it the right way, but well, I thought maybe it is better to base the networking on an asynchronous base instead of going synchronous from ground up and later refactoring everything for better async performance.

> **Attention**  
> This is **not** a finished product at all. I've published the project to show my progress on the implementation and for *(maybe)* collecting some useful feedback and improvement suggestions, so I get better into Rust.