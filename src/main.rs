#![allow(unused)]

mod base;
mod nat;
mod bool;
mod fin;
use base::*;
use nat::*;
use bool::*;
use fin::*;

// type of standard functions [A -> B]
trait Function<A,B> {}

// type of dependent functions [Π(x:A),B(x):U]or[A -> B:U]
trait DepFunc<A,U,B:Function<A,U>> {}


fn main() {

}
