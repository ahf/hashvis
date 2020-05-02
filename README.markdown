# Hashvis  [![Build Status](https://travis-ci.org/ahf/hashvis.svg?branch=master)](https://travis-ci.org/ahf/hashvis)

**A Rust application for deterministic generation of images.**

Hashvis is a small Rust application and library for generating very psychedelic
images from an input string. The images are generated in a deterministic manner
such that the same input string should always result in the same image being
generated.

Hashvis is inspired by OpenSSH's `VisualHostKey` where the OpenSSH client will
display a deterministic piece of ASCII art generated from the host keys when
you are trying to authenticate with a remote server. The idea behind this
feature is that humans have an easier way to remember visual impressions than
long, hex-encoded, fingerprints.

Example usage could be:

- OpenSSH host keys (like with `VisualHostKey` described above).
- OpenPGP public key fingerprints.
- Tor onion and next-generation onion public keys.

Currently Hashvis takes the user input string and passes it to the SHA3-256
hash function and uses this hash value to seed the ChaCha random number
generator. The random number generator is then used to generate a lot of 64-bit
floating-point values that are used to generate the expressions that is finally
used to plot the RGB image.

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

## Zooming In and Out

The `factor` argument that the Hashvis binary takes as an input allows you to
zoom in and out. A value smaller than 1.0 will zoom in, and anything larger
than 1.0 will zoom out of the normal view port. The default value is 1.0.

![31512-1](https://raw.githubusercontent.com/ahf/hashvis/master/misc/examples/31512-factor-1.png "SHA3(31512) with factor = 1 (default)")
![31512-4](https://raw.githubusercontent.com/ahf/hashvis/master/misc/examples/31512-factor-4.png "SHA3(31512) with factor = 4")

## Authors

- Alexander Færøy (<ahf@0x90.dk>).

## Interactive Twitter Bot

Matthew Garrett (@mjg59) wrote a very cool interactive Twitter bot which
fetches your Github SSH Public Key and executes Hashvis on the first key and
tweets out the generated picture.

You can write your Github username to the bot and it will tweet out your,
hopefully very unique, picture.

Check out the bot at https://twitter.com/github_idbot

## Gallery

![ahf](https://raw.githubusercontent.com/ahf/hashvis/master/misc/examples/ahf.png "SHA3(ahf)")
![Foobar](https://raw.githubusercontent.com/ahf/hashvis/master/misc/examples/Foobar.png "SHA3(Foobar)")
![1369](https://raw.githubusercontent.com/ahf/hashvis/master/misc/examples/1369.png "SHA3(1369)")
