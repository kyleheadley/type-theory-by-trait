use typed::*;
use arrow::*;
use darrow::*;

/// Composite Function [ A(B(x)) ]
pub struct Comp<A:FuncType,B:FuncType<T2=A::T1>>(A,B);
impl<A:FuncType,B:FuncType<T2=A::T1>> Typed for Comp<A,B> {type T=Arrow<B::T1,A::T2>;}
impl<A:FuncType,B:FuncType<T2=A::T1>,X:Typed<T=B::T1>> Func<X> for Comp<A,B> where
	B: Func<X>,
	A: Func<<B as Func<X>>::F>,
{type F=<A as Func<<B as Func<X>>::F>>::F;}

/// Constant function from any type T to value V
pub struct Const<T:Typed,V:Typed>(T,V);
impl<T1:Typed,T2:Typed,V:Typed<T=T2>> Typed for Const<T1,V> {type T=Arrow<T1,T2>;}
impl<T1:Typed,A:Typed<T=T1>,T2:Typed,V:Typed<T=T2>> Func<A> for Const<T1,V> {type F=V;}

/// Type Function from A to A->A
pub struct PolyArrow;
impl Typed for PolyArrow {type T=Arrow<Star,Star>;}
impl<A:Typed<T=Star>> Func<A> for PolyArrow {type F=Arrow<A,A>;}

/// Polymorphic identity fn
pub struct Id<T:Typed>(T);
impl<T:Typed> Typed for Id<T> {type T=Arrow<T,T>;}
impl<T:Typed,A:Typed<T=T>> Func<A> for Id<T> {type F=A;}
pub struct IdFn;
impl Typed for IdFn {type T=Pi<Star,PolyArrow>;}
impl<A:Typed<T=Star>> DFunc<A> for IdFn {type D=Id<A>;}
