# Tinylisp-rs

A tiny LISP interpreter running on Raspberry Pi Pico 2, based on [Robert-van-Engelen/tinylisp](https://github.com/Robert-van-Engelen/tinylisp). Written in Rust.

This project is a learning project. It includes most of the basic primitives of McCarthy's LISP and some Scheme's feature. Currently it does not support reading and parsing in stream, and can only interpret one line of code at a time. A `prelude.lisp` is loaded before reading input from USB serial port.

The file `memory.x` and parts of `.cargo/config.toml` are from [rp235x-hal](https://github.com/rp-rs/rp-hal)'s example repo.

The idea Also, coincidentally, a few weeks after I started the project, I found [MIT's SICP](https://ocw.mit.edu/courses/6-001-structure-and-interpretation-of-computer-programs-spring-2005/) course and was suprised by is clean philosophy of software engineering. (Sadly the course hasn't beed lectured since 2005, and the videos I found was recorded in 1986. Wish the course will be re-lectured like the Missing Semester.) I've bought some embedded devices last winter, so after I finished the overall strcuture of the project, I wondered if I can run LISP on this micro-controller like, for example, Lua or MuJS.
