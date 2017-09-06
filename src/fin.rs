use base::*;
use nat::*;

pub struct Fin<N:Type<Nat>> {}

pub struct FinNum<N:Type<Nat>,F:Type<Nat>>(pub N,pub F);
impl<F:Type<Nat>,N:Type<Nat>> Type<Fin<F>> for FinNum<N,F> where
	GreaterThan: Judge2<F,N>
{}

pub struct FMax;
impl<N:Type<Nat>> Func<N> for FMax where
	GreaterThan: Judge2<Succ<N>,N>
{type F=FinNum<N,Succ<N>>;}

pub trait FMaxFn<N:Type<Nat>> { type F:Type<Fin<Succ<N>>>;}
pub struct TypedFMax;
impl<N:Type<Nat>> FMaxFn<N> for TypedFMax where
	GreaterThan: Judge2<Succ<N>,N>
{type F=FinNum<N,Succ<N>>;}
