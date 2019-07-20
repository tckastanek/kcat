kcat [![Build Status](https://travis-ci.org/tckastanek/kcat.svg?branch=master)](https://travis-ci.org/tckastanek/kcat)
===

Key Cat

It's pretty much just the `cat` command, but with syntax highlighting and the ability to key path into JSON files.

## How do I use it?
Install it with `cargo`:

`$ cargo install kcat`

Then you can use it!:

```
USAGE:
       kcat [FLAGS] [OPTIONS] [FILE]...
   
   FLAGS:
       -c, --cache-theme             Cache the parsed theme file
       -h, --help                    Prints help information
       -L, --list-embedded-themes    Lists themes present in the executable
       -l, --list-file-types         Lists supported file types
       -e, --no-default-syntaxes     Doesn't load default syntaxes, intended for use with --extra-syntaxes.
       -V, --version                 Prints version information
   
   OPTIONS:
       -k, --KEY PATH <KEY PATH>               key path
       -s, --extra-syntaxes <SYNTAX FOLDER>    Additional folder to search for .sublime-syntax files in.
       -t, --theme-file <THEME FILE>           Theme file to use. May be a path, or an embedded theme. Embedded themes will
                                               take precedence. Default: base16-ocean.dark
   
   ARGS:
       <FILE>...    file path
```

Key paths work like you'd expect. A typical command would look like:

`$ kcat package.json -k scripts.start`

## Does it work on Mac?
Sure does.

## Does it work on Linux/Windows?
I have no idea--probably not, in fact. But let me know!

## Is it fast?
It's probably fast enough.

## Does it have all the regular features of `cat`?
Probably not.


Relies heavily on [syntect](https://github.com/trishume/syntect) for both highlighting and most of the implementation.  