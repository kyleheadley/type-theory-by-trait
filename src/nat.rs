use base::*;

pub struct Nat(());
impl Type for Nat {type T=Star;}

pub struct Zero;
impl Type for Zero {type T=Nat;}

pub struct Succ<N:Type<T=Nat>>(pub N);
impl<N:Type<T=Nat>> Type for Succ<N> {type T=Nat;}

pub struct Pred;
impl Arrow for Pred {type T1=Nat; type T2=Nat;}
impl<N:Type<T=Nat>> Func<Succ<N>> for Pred { type F=N; }

pub struct GreaterThan;
impl<N:Type<T=Nat>> Judge2<Succ<N>,Zero> for GreaterThan {}
impl<N1:Type<T=Nat>,N2:Type<T=Nat>> Judge2<Succ<N1>,Succ<N2>> for GreaterThan where
  GreaterThan: Judge2<N1,N2>
{}

pub struct Plus;
impl Arrow2 for Plus {type T1=Nat; type T2=Nat; type T3=Nat;}
impl<N:Type<T=Nat>> Func2<Zero,N> for Plus { type F = N; }
impl<N1:Type<T=Nat>,N2:Type<T=Nat>> Func2<Succ<N1>,N2> for Plus where
  Plus: Func2<N1,N2>,
{ type F = Succ<<Plus as Func2<N1,N2>>::F>; }
