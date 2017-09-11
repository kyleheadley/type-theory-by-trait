use typed::*;
use arrow::*;

/// Type of natural numbers
pub struct Nat;
impl Typed for Nat {type T=Star;}

/// Zero : Nat
pub struct Zero;
impl Typed for Zero {type T=Nat;}
/// Succ<N> : Nat
pub struct Succ<N:Typed<T=Nat>>(N);
impl<N:Typed<T=Nat>> Typed for Succ<N> {type T=Nat;}
/// Functional constructor for Succ
pub struct SuccFn;
impl Typed for SuccFn {type T=Arrow<Nat,Nat>;}
impl<N:Typed<T=Nat>> Func<N> for SuccFn {type F=Succ<N>;}

/// Predecessor function
pub struct Pred;
impl Typed for Pred {type T=Arrow<Nat,Nat>;}
impl<N:Typed<T=Nat>> Func<Succ<N>> for Pred { type F=N; }

/// Judgement of N1 > N2
pub struct GreaterThan;
impl<N:Typed<T=Nat>> Judge2<Succ<N>,Zero> for GreaterThan {}
impl<N1:Typed<T=Nat>,N2:Typed<T=Nat>> Judge2<Succ<N1>,Succ<N2>> for GreaterThan where
  GreaterThan: Judge2<N1,N2>
{}

/// Addition function for Nat
pub struct Plus;
impl Typed for Plus {type T=Arrow2<Nat,Nat,Nat>;}
impl<N:Typed<T=Nat>> Func2<Zero,N> for Plus { type F = N; }
impl<N1:Typed<T=Nat>,N2:Typed<T=Nat>> Func2<Succ<N1>,N2> for Plus where
  Plus: Func2<N1,N2>,
{ type F = Succ<<Plus as Func2<N1,N2>>::F>; }
