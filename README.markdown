# Hashvis  [![Build Status](https://travis-ci.org/ahf/hashvis.svg?branch=master)](https://travis-ci.org/ahf/hashvis)

**A Rust application for deterministically generation of images.**

## Building Hashvis

To download and build Hashvis:

    $ git clone https://github.com/ahf/hashvis.git
    $ cd hashvis
    $ cargo build

You can try running the application using:

    $ cargo run --bin hashvis misc/examples/yourusername.png yourusername 256
        Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
         Running `target/debug/hashvis misc/examples/yourusername.png yourusername`
    Generating image with SHA3("yourusername") in misc/examples/yourusername.png ...
    R: f(x, y) = cos(pi * sin(pi * cos(pi * tan(pi * sin(pi * sin(pi * y) * y)) * y)))
    G: f(x, y) = tan(pi * tan(pi * cos(pi * x * tan(pi * y) * x * x)) * tan(pi * sin(pi * sin(pi * x) * sin(pi * y))))
    B: f(x, y) = tan(pi * x)

Feel free to submit a pull request if you think your generated image looks
spectacular.

## Testing Hashvis

To run the minimal test suite:

    $ cargo test

## Authors

- Alexander Færøy (<ahf@0x90.dk>).

## Gallery

![ahf](https://raw.githubusercontent.com/ahf/hashvis/master/misc/examples/ahf.png "SHA3(ahf)")
![Foobar](https://raw.githubusercontent.com/ahf/hashvis/master/misc/examples/Foobar.png "SHA3(Foobar)")
![1369](https://raw.githubusercontent.com/ahf/hashvis/master/misc/examples/1369.png "SHA3(1369)")
