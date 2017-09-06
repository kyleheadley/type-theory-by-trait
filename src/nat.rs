use base::*;

pub struct Nat(());

pub struct Zero;
impl Type<Nat> for Zero {}

pub struct Succ<N>(pub N);
impl<N:Type<Nat>> Type<Nat> for Succ<N> {}

pub struct Pred;
impl Type<Arrow<Nat,Nat>> for Pred {}
impl<N:Type<Nat>> Func<Nat,Nat,Succ<N>> for Pred { type F=N; }

pub struct GreaterThan;
impl<N:Type<Nat>> Judge2<Succ<N>,Zero> for GreaterThan {}
impl<N1:Type<Nat>,N2:Type<Nat>> Judge2<Succ<N1>,Succ<N2>> for GreaterThan where
  GreaterThan: Judge2<N1,N2>
{}

pub trait NatOp<N1:Type<Nat>,N2:Type<Nat>> {type F:Type<Nat>;}

pub struct Plus;
impl Type<Arrow2<Nat,Nat,Nat>> for Plus {}
impl<N:Type<Nat>> Func2<Nat,Nat,Nat,Zero,N> for Plus { type F = N; }
impl<N1:Type<Nat>,N2:Type<Nat>> Func2<Nat,Nat,Nat,Succ<N1>,N2> for Plus where
  Plus: Func2<Nat,Nat,Nat,N1,N2>,
{ type F = Succ<<Plus as Func2<Nat,Nat,Nat,N1,N2>>::F>; }
impl<N1:Type<Nat>,N2:Type<Nat>> NatOp<N1,N2> for Plus where
  Plus: Func2<Nat,Nat,Nat,N1,N2>
{type F=<Plus as Func2<Nat,Nat,Nat,N1,N2>>::F;}
