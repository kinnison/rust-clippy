// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(clippy::all)]
#![allow(unused, clippy::needless_pass_by_value)]
#![feature(associated_type_defaults)]

type Alias = Vec<Vec<Box<(u32, u32, u32, u32)>>>; // no warning here

const CST: (u32, (u32, (u32, (u32, u32)))) = (0, (0, (0, (0, 0))));
static ST: (u32, (u32, (u32, (u32, u32)))) = (0, (0, (0, (0, 0))));

struct S {
    f: Vec<Vec<Box<(u32, u32, u32, u32)>>>,
}

struct TS(Vec<Vec<Box<(u32, u32, u32, u32)>>>);

enum E {
    Tuple(Vec<Vec<Box<(u32, u32, u32, u32)>>>),
    Struct { f: Vec<Vec<Box<(u32, u32, u32, u32)>>> },
}

impl S {
    const A: (u32, (u32, (u32, (u32, u32)))) = (0, (0, (0, (0, 0))));
    fn impl_method(&self, p: Vec<Vec<Box<(u32, u32, u32, u32)>>>) {}
}

trait T {
    const A: Vec<Vec<Box<(u32, u32, u32, u32)>>>;
    type B = Vec<Vec<Box<(u32, u32, u32, u32)>>>;
    fn method(&self, p: Vec<Vec<Box<(u32, u32, u32, u32)>>>);
    fn def_method(&self, p: Vec<Vec<Box<(u32, u32, u32, u32)>>>) {}
}

fn test1() -> Vec<Vec<Box<(u32, u32, u32, u32)>>> {
    vec![]
}

fn test2(_x: Vec<Vec<Box<(u32, u32, u32, u32)>>>) {}

fn test3() {
    let _y: Vec<Vec<Box<(u32, u32, u32, u32)>>> = vec![];
}

#[repr(C)]
struct D {
    // should not warn, since we don't have control over the signature (#3222)
    test4: extern "C" fn(
        itself: &D,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        g: usize,
        h: usize,
        i: usize,
    ),
}

fn main() {}
