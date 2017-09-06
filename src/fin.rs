use base::*;
use nat::*;

pub struct Fin<N:Type<Nat>>(N);

pub struct FinNum<N:Type<Nat>,F:Type<Nat>>(pub N,pub F);
impl<F:Type<Nat>,N:Type<Nat>> Type<Fin<F>> for FinNum<N,F> where
	GreaterThan: Judge2<F,N>
{}

pub struct FMax;
impl<N:Type<Nat>> Type<Arrow<Nat,Fin<Succ<N>>>> for FMax {}
impl<N:Type<Nat>> Func<Nat,Fin<Succ<N>>,N> for FMax where
	GreaterThan: Judge2<Succ<N>,N>
{type F=FinNum<N,Succ<N>>;}
