#![allow(unused)]

mod typed;
mod util;
mod arrow;
mod darrow;
mod pair;
mod abspair;
mod dpair;
mod union;
mod nat;
mod bool;
mod fin;
mod stlc;
use typed::*;
use util::*;
use arrow::*;
use darrow::*;
use pair::*;
use abspair::*;
use union::*;
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

// Type family
struct NatToBoolOrNat;
impl Typed for NatToBoolOrNat {type T=Arrow<Nat,Star>;}
impl<N:Typed<T=Nat>> Func<N> for NatToBoolOrNat where
	IsGreater: Func2<Succ<Succ<Succ<Zero>>>,N>,
	BoolOrNat: Func<<IsGreater as Func2<Succ<Succ<Succ<Zero>>>,N>>::F>
{type F=<BoolOrNat as Func<
	<IsGreater as Func2<Succ<Succ<Succ<Zero>>>,N>>::F
>>::F;}

// Dependent function
struct TrueOr2;
impl Typed for TrueOr2 {type T=Pi<Nat,NatToBoolOrNat>;}
impl DFunc<Zero> for TrueOr2 {type D=Zero;}
// impl<N:Typed<T=Nat>> DFunc<Succ<N>> for TrueOr2 where
// 	IsGreater: Func2<Succ<Succ<Zero>>,N>,
// 	BoolOrNat: Func<<IsGreater as Func2<Succ<Succ<Zero>>,N>>::F>
// {type D=True;}
impl DFunc<Succ<Zero>> for TrueOr2 {type D=Succ<Succ<Zero>>;}
impl DFunc<Succ<Succ<Zero>>> for TrueOr2 {type D=Zero;}
impl DFunc<Succ<Succ<Succ<Zero>>>> for TrueOr2 {type D=False;}

fn main() {

}
