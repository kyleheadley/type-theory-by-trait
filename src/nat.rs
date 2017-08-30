use base::*;

pub trait Nat {}
impl BaseType for Nat {}

pub struct Zero;
impl Nat for Zero {}

pub struct Succ<N>(pub N);
impl<N:Nat> Nat for Succ<N> {}

pub struct Pred;
impl<N:Nat> Func<Succ<N>> for Pred { type F=N; }

pub struct GreaterThan;
impl<N:Nat> Judge2<Succ<N>,Zero> for GreaterThan {}
impl<N1:Nat,N2:Nat> Judge2<Succ<N1>,Succ<N2>> for GreaterThan where
  GreaterThan: Judge2<N1,N2>
{}

pub struct Plus;
impl<N:Nat> Func2<Zero,N> for Plus { type F = N; }
impl<N1:Nat,N2:Nat> Func2<Succ<N1>,N2> for Plus where
  Plus: Func2<N1,N2>,
{ type F = Succ<<Plus as Func2<N1,N2>>::F>; }

pub trait PlusFn<N1:Nat,N2:Nat> {type F:Nat;}
pub struct TypedPlus;
impl<N:Nat> PlusFn<Zero,N> for TypedPlus { type F = N; }
impl<N1:Nat,N2:Nat> PlusFn<Succ<N1>,N2> for TypedPlus where
  TypedPlus: PlusFn<N1,N2>,
{ type F = Succ<<TypedPlus as PlusFn<N1,N2>>::F>; }
