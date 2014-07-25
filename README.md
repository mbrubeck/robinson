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

Status
------

Currently implemented:

* Parse a small subset of HTML and build a DOM tree.
* Parse a small subset of CSS.
* Perform selector matching to apply styles to elements.

Coming soon, I hope:

* Basic block and inline layout.
* Paint text and boxes.
* Load resources from network or filesystem.
