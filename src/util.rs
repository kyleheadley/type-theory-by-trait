use base::*;
use arrow::*;
use depend::*;

/// Composite Function [ A(B(x)) ]
pub struct Comp<A:FuncType,B:FuncType<T2=A::T1>>(A,B);
impl<A:FuncType,B:FuncType<T2=A::T1>> Type for Comp<A,B> {type T=Arrow<B::T1,A::T2>;}
impl<A:FuncType,B:FuncType<T2=A::T1>,X:Type<T=B::T1>> Func<X> for Comp<A,B> where
	B: Func<X>,
	A: Func<<B as Func<X>>::F>,
{type F=<A as Func<<B as Func<X>>::F>>::F;}

/// Constant function from any type T to value V
pub struct Const<T:Type,V:Type>(T,V);
impl<T1:Type,T2:Type,V:Type<T=T2>> Type for Const<T1,V> {type T=Arrow<T1,T2>;}
impl<T1:Type,A:Type<T=T1>,T2:Type,V:Type<T=T2>> Func<A> for Const<T1,V> {type F=V;}

/// Type Function from A to A->A
pub struct PolyArrow;
impl Type for PolyArrow {type T=Arrow<Star,Star>;}
impl<A:Type<T=Star>> Func<A> for PolyArrow {type F=Arrow<A,A>;}

/// Polymorphic identity fn
pub struct Id<T:Type>(T);
impl<T:Type> Type for Id<T> {type T=Arrow<T,T>;}
impl<T:Type,A:Type<T=T>> Func<A> for Id<T> {type F=A;}
pub struct IdFn;
impl Type for IdFn {type T=Pi<Star,PolyArrow>;}
impl<A:Type<T=Star>> DFunc<A> for IdFn {type D=Id<A>;}
