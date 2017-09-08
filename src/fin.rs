use base::*;
use util::*;
use nat::*;
use arrow::*;
use depend::*;

pub struct Fin<N:Type<T=Nat>>(N);
impl<N:Type<T=Nat>> Type for Fin<N>{type T=Star;}
pub struct FinFn;
impl Type for FinFn {type T=Arrow<Nat,Star>;}
impl<N:Type<T=Nat>> Func<N> for FinFn {type F=Fin<N>;}

pub struct FinNum<N:Type<T=Nat>,F:Type<T=Nat>>(pub N,pub F);
impl<F:Type<T=Nat>,N:Type<T=Nat>> Type for FinNum<N,F> where
	GreaterThan: Judge2<F,N>
{type T=Fin<F>;}

pub struct FMax;
impl PiType for FMax {type T1=Nat; type F=Comp<FinFn,SuccFn>;}
impl<N:Type<T=Nat>> DFunc<N> for FMax where
	GreaterThan: Judge2<Succ<N>,N>
{type D=FinNum<N,Succ<N>>;}
