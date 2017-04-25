# Mobula

A [ray tracer] written in rust following
[_Ray Tracing in One Weekend_][rtiow].

The default output is:

![current output](https://github.com/isaacazuelos/mobula/blob/master/out.png)

The name is the latin name of a [taxonomic family][wiki].

[rtiow]: http://psgraphics.blogspot.ca/2016/01/new-mini-book-ray-tracing-in-one-weekend.html
[ray tracer]: https://en.wikipedia.org/wiki/Ray_tracing_(graphics)
[wiki]: https://en.wikipedia.org/wiki/Mobula

## Why?

I wrote a half-broken ray tracer as part of a computer graphics course years
ago. I've wanted to do a better job since — and it's a fun excuse to use rust.

## Requirements and Installation

You'll need [`cargo`][cargo] and rust stable 1.16 (at least that's what I
used.)

You can fetch the project with `git`:

```sh
$ git clone https://github.com/isaacazuelos/mobula
```

and build it with [Cargo][]:

```sh
$ cd mobula
$ cargo install
$ mobula
```

[cargo]: http://doc.crates.io

## Usage

All it does is render the static image — there are no command line options.

## License

Since I didn't write the original code, I'm honestly not sure about the legal
status of this, so I didn't include a license.
