use prim::*;

pub trait Nat {}
impl Prim for Nat {}

pub struct Zero;
impl Nat for Zero {}

pub struct Succ<N>(pub N);
impl<N:Nat> Nat for Succ<N> {}

pub trait Pred<N:Nat> { type F; }
impl<N:Nat> Pred<Succ<N>> for Nat { type F=N; }

pub trait GreaterThan<N1,N2> {}
impl<N:Nat> GreaterThan<Zero,Succ<N>> for Nat {}
impl<N1:Nat,N2:Nat> GreaterThan<Succ<N1>,Succ<N2>> for Nat where
  Nat: GreaterThan<N1,N2>
{}

pub trait Plus<N1:Nat,N2:Nat> { type F:Nat; }
impl<N:Nat> Plus<Zero,N> for Nat { type F = N; }
impl<N1:Nat,N2:Nat> Plus<Succ<N1>,N2> for Nat where
  Nat: Plus<N1,N2>
{ type F = Succ<<Nat as Plus<N1,N2>>::F>; }
