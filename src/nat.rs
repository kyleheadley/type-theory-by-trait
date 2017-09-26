use typed::*;
use arrow::*;
use bool::*;

/// Type of natural numbers
#[derive(Debug)]
pub struct Nat;
impl Typed for Nat {
	fn reflect() -> String {format!("Nat")}
	type T=Star;
}

/// Zero : Nat
pub struct Zero;
impl Typed for Zero {
	fn reflect() -> String {format!("0")}
	type T=Nat;
}
/// Succ<N> : Nat
pub struct Succ<N:Typed<T=Nat>>(pub N);
impl<N:Typed<T=Nat>> Typed for Succ<N> {
	fn reflect() -> String {format!("{}",N::reflect().parse::<usize>().unwrap())}
	type T=Nat;
}
/// Functional constructor for Succ
pub struct SuccFn;
impl Typed for SuccFn {
	fn reflect() -> String {format!("succ()")}
	type T=Arrow<Nat,Nat>;
}
impl<N:Typed<T=Nat>> Func<N> for SuccFn {type F=Succ<N>;}

/// Predecessor function
pub struct Pred;
impl Typed for Pred {
	fn reflect() -> String {format!("pred()")}
	type T=Arrow<Nat,Nat>;
}
impl<N:Typed<T=Nat>> Func<Succ<N>> for Pred { type F=N; }

/// Judgement of N1 > N2
pub struct GreaterThan;
impl<N:Typed<T=Nat>> Judge2<Succ<N>,Zero> for GreaterThan {}
impl<N1:Typed<T=Nat>,N2:Typed<T=Nat>> Judge2<Succ<N1>,Succ<N2>> for GreaterThan where
  GreaterThan: Judge2<N1,N2>
{}

/// Boolean function of N1 > N2
pub struct IsGreater;
impl Typed for IsGreater {
	fn reflect() -> String {format!(">")}
	type T=Arrow2<Nat,Nat,Bool>;
}
impl<N:Typed<T=Nat>> Func2<Zero,N> for IsGreater {type F=False;}
impl<N:Typed<T=Nat>> Func2<Succ<N>,Zero> for IsGreater {type F=True;}
impl<N1:Typed<T=Nat>,N2:Typed<T=Nat>> Func2<Succ<N1>,Succ<N2>> for IsGreater where
	IsGreater: Func2<N1,N2>
{type F=<IsGreater as Func2<N1,N2>>::F;}

/// Boolean function of N1 ≥ N2
pub struct IsGreaterEq;
impl Typed for IsGreaterEq {
	fn reflect() -> String {format!("≥")}
	type T=Arrow2<Nat,Nat,Bool>;
}
impl<N:Typed<T=Nat>> Func2<Zero,Succ<N>> for IsGreaterEq {type F=False;}
impl<N:Typed<T=Nat>> Func2<N,Zero> for IsGreaterEq {type F=True;}
impl<N1:Typed<T=Nat>,N2:Typed<T=Nat>> Func2<Succ<N1>,Succ<N2>> for IsGreaterEq where
	IsGreaterEq: Func2<N1,N2>
{type F=<IsGreaterEq as Func2<N1,N2>>::F;}

/// Boolean function of N1 = N2
pub struct IsEqual;
impl Typed for IsEqual {
	fn reflect() -> String {format!("=")}
	type T=Arrow2<Nat,Nat,Bool>;
}
impl Func2<Zero,Zero> for IsEqual {type F=True;}
impl<N:Typed<T=Nat>> Func2<Succ<N>,Zero> for IsEqual {type F=False;}
impl<N:Typed<T=Nat>> Func2<Zero,Succ<N>> for IsEqual {type F=False;}
impl<N1:Typed<T=Nat>,N2:Typed<T=Nat>> Func2<Succ<N1>,Succ<N2>> for IsEqual where
	IsEqual: Func2<N1,N2>,
{type F=<IsEqual as Func2<N1,N2>>::F;}

/// Addition function for Nat
pub struct Plus;
impl Typed for Plus {
	fn reflect() -> String {format!("+")}
	type T=Arrow2<Nat,Nat,Nat>;
}
impl<N:Typed<T=Nat>> Func2<Zero,N> for Plus { type F = N; }
impl<N1:Typed<T=Nat>,N2:Typed<T=Nat>> Func2<Succ<N1>,N2> for Plus where
  Plus: Func2<N1,N2>,
{ type F = Succ<<Plus as Func2<N1,N2>>::F>; }
