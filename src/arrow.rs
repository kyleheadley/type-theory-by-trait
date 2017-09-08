use base::*;

/// Type of functions of one variable
pub struct Arrow<T1,T2>(T1,T2);
impl<T1,T2> Type for Arrow<T1,T2> {type T=Star;}
pub trait FuncType {type T1; type T2;}
impl<T1,T2,A:Type<T=Arrow<T1,T2>>> FuncType for A {type T1=T1; type T2=T2;}
/// Type of functions of two variables
pub struct Arrow2<T1,T2,T3>(T1,T2,T3);
impl<T1,T2,T3> Type for Arrow2<T1,T2,T3> {type T=Star;}
pub trait Func2Type {type T1; type T2; type T3;}
impl<T1,T2,T3,A:Type<T=Arrow2<T1,T2,T3>>> Func2Type for A {type T1=T1; type T2=T2; type T3=T3;}

/// (Partial) function of one variable
pub trait Func<A:Type<T=Self::T1>> : FuncType {type F:Type<T=Self::T2>;}
/// (Partial) function of two variables
pub trait Func2<A:Type<T=Self::T1>,B:Type<T=Self::T2>> : Func2Type {type F:Type<T=Self::T3>;}

// untyped Func works as an alternate, but with no constraints
// pub trait Func<A> {type F;}
// pub trait Func2<A,B> {type F;}
