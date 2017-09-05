use base::*;
use nat::*;

pub trait Fin<N:Type<Nat>> {}
//impl<N:Type<Nat>,T:Type<Fin<N>>> Fin<N> for T {}

pub struct FinNum<N:Type<Nat>,F:Type<Nat>>(pub N,pub F);
impl<F:Type<Nat>,N:Type<Nat>> Type<Fin<F>> for FinNum<N,F> where
	GreaterThan: Judge2<Nat,Nat,F,N>
{}

pub struct FMax;
impl<N:Type<Nat>> Func<Nat,Fin<Succ<N>>,N> for FMax where
	GreaterThan: Judge2<Nat,Nat,Succ<N>,N>
{type F=FinNum<N,Succ<N>>;}

// pub trait FMaxFn<N:Nat> { type F:Fin<Succ<N>>;}
// pub struct TypedFMax;
// impl<N:Nat> FMaxFn<N> for TypedFMax where
// 	GreaterThan: Judge2<Nat,Nat,Succ<N>,N>
// {type F=FinNum<N,Succ<N>>;}
