use base::*;
use util::*;
use nat::*;
use arrow::*;
use depend::*;

/// Type of the N finite integers
pub struct Fin<N:Type<T=Nat>>(N);
impl<N:Type<T=Nat>> Type for Fin<N>{type T=Star;}
/// Functional constructor for Fin
pub struct FinFn;
impl Type for FinFn {type T=Arrow<Nat,Star>;}
impl<N:Type<T=Nat>> Func<N> for FinFn {type F=Fin<N>;}

/// Constructor for a finite integer
pub struct FinNum<N:Type<T=Nat>,F:Type<T=Nat>>(pub N,pub F);
impl<F:Type<T=Nat>,N:Type<T=Nat>> Type for FinNum<N,F> where
	GreaterThan: Judge2<F,N>
{type T=Fin<F>;}

/// Function returning max value of Fin<N> type
pub struct FMax;
impl Type for FMax {type T=Pi<Nat,Comp<FinFn,SuccFn>>;}
impl<N:Type<T=Nat>> DFunc<N> for FMax where
	GreaterThan: Judge2<Succ<N>,N>
{type D=FinNum<N,Succ<N>>;}
