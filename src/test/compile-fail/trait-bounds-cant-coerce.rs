// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Foo {
}

fn a(_x: ~Foo:Owned) {
}

fn b(_x: ~Foo:Owned+Copy) {
}

fn c(x: ~Foo:Const+Owned) {
    b(x); //~ ERROR expected bounds `Copy+Owned`
}

fn d(x: ~Foo) {
    a(x); //~ ERROR found no bounds
}

fn main() { }
