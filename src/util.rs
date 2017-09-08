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

/// Constant function from any type T to V
pub struct Const<T,V>(T,V);
impl<T1,T2,V:Type<T=T2>> Type for Const<T1,V> {type T=Arrow<T1,T2>;}
impl<T1,A:Type<T=T1>,T2,V:Type<T=T2>> Func<A> for Const<T1,V> {type F=V;}

/// Polymorphic identity fn
pub struct PolyFn;
impl Type for PolyFn {type T=Arrow<Star,Star>;}
impl<A:Type<T=Star>> Func<A> for PolyFn {type F=Arrow<A,A>;}
pub struct IdFn<T>(T);
impl<T> Type for IdFn<T> {type T=Arrow<T,T>;}
impl<T,A:Type<T=T>> Func<A> for IdFn<T> {type F=A;}
#[allow(non_camel_case_types)]
pub struct id;
impl Type for id {type T=Pi<Star,PolyFn>;}
impl<A:Type<T=Star>> DFunc<A> for id {type D=IdFn<A>;}
