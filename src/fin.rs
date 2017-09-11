use typed::*;
use util::*;
use nat::*;
use arrow::*;
use darrow::*;

/// Type of the N finite integers
pub struct Fin<N:Typed<T=Nat>>(N);
impl<N:Typed<T=Nat>> Typed for Fin<N>{type T=Star;}
/// Functional constructor for Fin
pub struct FinFn;
impl Typed for FinFn {type T=Arrow<Nat,Star>;}
impl<N:Typed<T=Nat>> Func<N> for FinFn {type F=Fin<N>;}

/// Constructor for a finite integer
pub struct FinNum<N:Typed<T=Nat>,F:Typed<T=Nat>>(pub N,pub F);
impl<F:Typed<T=Nat>,N:Typed<T=Nat>> Typed for FinNum<N,F> where
	GreaterThan: Judge2<F,N>
{type T=Fin<F>;}

/// Function returning max value of Fin<N> type
pub struct FMax;
impl Typed for FMax {type T=Pi<Nat,Comp<FinFn,SuccFn>>;}
impl<N:Typed<T=Nat>> DFunc<N> for FMax where
	GreaterThan: Judge2<Succ<N>,N>
{type D=FinNum<N,Succ<N>>;}
