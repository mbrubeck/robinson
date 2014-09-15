Robinson
========

A toy web rendering engine written in the Rust language, by Matt Brubeck
(mbrubeck@limpet.net).

I'm writing this code purely for educational purposes. My goal is to create an
incomplete but extremely simple engine as a way to learn more about basic
implementation techniques, *without* worrying about complications like:

* <s>Real-world usability</s>
* <s>Standards compliance</s>
* <s>Performance and efficiency</s>
* <s>Interoperability</s>

These are all important goals, but there are other projects working on them.
By ignoring them completely, this project can focus on being as simple and
easy-to-understand as possible.

Why create a simple—but useless—toy rendering engine? Mostly because I
personally want to learn how to do it. If I succeed, I also hope that other
people can learn from my code by reading or modifying it, or learn from my
experience as they set out to build their own toy browser engines.

For more details see [Let's build a browser engine!][blog], a series of
how-to articles based on this project.

[blog]: http://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html

Status
------

Currently implemented:

* Parse a small subset of HTML and build a DOM tree.
* Parse a small subset of CSS.
* Perform selector matching to apply styles to elements.
* Basic block layout.

Coming soon, I hope:

* Inline layout.
* Paint text and boxes.
* Load resources from network or filesystem.

Instructions
------------

1. Install [Rust](http://www.rust-lang.org/) 0.12.0-pre or newer. (Nightly
   builds are recommended.)

2. Clone the robinson source code from https://github.com/mbrubeck/robinson

3. Run `make` to build robinson, and `make run` to run it.

By default, robinson will load test.html and test.css from the `examples`
directory.  You can use the `--html` and `--css` arguments to the robinson
executable to change the input files:

    ./bin/main --html file.html --css file.css
