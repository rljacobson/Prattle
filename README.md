# Prattle: A Pratt Parser & Parser Generator in Rust

This is a work in progress and has not yet reached a useful level of maturity. Check back soon!

## What is it?

A parser reads in text and recognizes it, transforming it into an internal format the computer can
understand. The Pratt parsing algorithm is one of the most useful and elegant of the parsing
algorithms yet has suffered in obscurity for decades. It is especially adept at parsing expression
grammars. To read more about it, check out these resources:

* TODO
* TODO

## Prerequisits

The great thing about Rust is that Cargo generally takes care of all of this for you. Consult the
Cargo.toml file for a list of dependencies. 

## Usage

TODO

### Which is it, a parser or a parser generator?

It is either, or both. Most Pratt parser designs hard-code the operator data into the source code
for no real benefit. But doing so severely limits the flexibility of the resulting parser. What
makes one Pratt parser different from another is often only the operator data. If you separate the
operator data from the code, you get a universal expression parser with all the same efficiency and
ease of use of any other Pratt parser. 

But maybe you *want* the inflexibility of hard-coded operator data for some reason. Then use
Prattle's code generator to write it for you. 

## Authors and License
 
 Copyright (C) 2019 Robert Jacobson
 
 Released under the MIT license. 
