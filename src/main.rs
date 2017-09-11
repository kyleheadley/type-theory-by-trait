#![allow(unused)]

mod typed;
mod util;
mod arrow;
mod darrow;
mod pair;
mod nat;
mod bool;
mod fin;
use typed::*;
use util::*;
use arrow::*;
use darrow::*;
use pair::*;
use nat::*;
use bool::*;
use fin::*;

// Type family
struct BoolOrNat;
impl Typed for BoolOrNat {type T=Arrow<Bool,Star>;}
impl Func<False> for BoolOrNat {type F=Bool;}
impl Func<True> for BoolOrNat {type F=Nat;}

// Dependent function
struct FalseOr3;
impl Typed for FalseOr3 {type T=Pi<Bool,BoolOrNat>;}
impl DFunc<False> for FalseOr3 {type D=False;}
impl DFunc<True> for FalseOr3 {type D=Succ<Succ<Succ<Zero>>>;}

fn main() {

}
