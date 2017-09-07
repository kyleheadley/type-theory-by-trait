use base::*;
use nat::*;

pub struct Fin<N:Type<T=Nat>>(N);

pub struct FinNum<N:Type<T=Nat>,F:Type<T=Nat>>(pub N,pub F);
impl<F:Type<T=Nat>,N:Type<T=Nat>> Type for FinNum<N,F> where
	GreaterThan: Judge2<F,N>
{type T=Fin<F>;}

// pub struct FMax;
// impl<N:Type<T=Nat>> Arrow<Nat,Fin<Succ<N>>> for FMax {}
// impl<N:Type<T=Nat>> Func<N> for FMax where
// 	//GreaterThan: Judge2<Succ<N>,N>
// {type F=FinNum<N,Succ<N>>;}
