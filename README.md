# xi-aurelius [![Build Status](https://travis-ci.com/euclio/xi-aurelius.svg?branch=master)](https://travis-ci.com/euclio/xi-aurelius)

A markdown preview server for [xi-editor] using [aurelius].

## Installation

xi-aurelius requires [stable Rust].

> There is no commitment
> to support older versions of Rust. While older versions may work, the latest
> stable version is recommended.

Installation is simple:

```sh
$ make && make install
```

This will install the plugin to the appropriate directory for xi to find it. On
startup, xi will automatically start the plugin.

[aurelius]: https://github.com/euclio/aurelius
[xi-editor]: https://xi-editor.github.io/xi-editor/
[stable Rust]: https://rustup.rs/
