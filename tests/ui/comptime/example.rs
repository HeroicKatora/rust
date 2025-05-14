//@ check-pass

#![feature(type_alias_impl_trait)]

use std::comptime::Context;

#[comptime]
trait Comptime {
    type ThisIsAResult;
}

const fn build_me(_: Context) -> &'static str {
    "struct Foobar; type ThisIsAResult = Foobar;"
}

type BuildAst = impl Fn(Context) -> &'static str;

#[define_opaque(BuildAst)]
fn _definer() -> BuildAst { build_me }

type Foobar = <BuildAst as Comptime>::ThisIsAResult;

fn main() {
    let _: Option<Foobar> = None;
}
