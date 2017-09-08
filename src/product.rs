use base::*;
use arrow::*;
use depend::*;

/// Type of empty value
pub struct UnitType;
impl Type for UnitType {type T=Star;}

/// The empty value
pub struct Unit;
impl Type for Unit {type T=UnitType;}

/// Type of pairs
pub struct Product<T1:Type,T2:Type>(T1,T2);
impl<A:Type,B:Type> Type for Product<A,B> {type T=Star;}

/// Pairs of values
pub struct Pair<A:Type,B:Type>(A,B);
impl<T1:Type,T2:Type,A:Type<T=T1>,B:Type<T=T2>> Type for Pair<A,B> {type T=Product<T1,T2>;}

/// Projection of first value (parameterized by value type)
pub struct Fst<T1:Type,T2:Type>(T1,T2);
impl<T1:Type,T2:Type> Type for Fst<T1,T2> {type T=Arrow<Product<T1,T2>,T1>;}
impl<T1:Type,T2:Type,A:Type<T=T1>,B:Type<T=T2>> Func<Pair<A,B>> for Fst<T1,T2> {type F=A;}

/// Projection of second value (parameterized by value type)
pub struct Snd<T1:Type,T2:Type>(T1,T2);
impl<T1:Type,T2:Type> Type for Snd<T1,T2> {type T=Arrow<Product<T1,T2>,T2>;}
impl<T1:Type,T2:Type,A:Type<T=T1>,B:Type<T=T2>> Func<Pair<A,B>> for Snd<T1,T2> {type F=B;}

// PairRec<A,B>:Π(C:U)[(A->B->C)->(AxB)->C]
pub struct PairRecFam<A,B>(A,B);
impl<A:Type,B:Type> Type for PairRecFam<A,B> {type T=Arrow<Star,Star>;}
impl<A:Type,B:Type,C:Type<T=Star>> Func<C> for PairRecFam<A,B> {type F=Arrow2<Arrow2<A,B,C>,Product<A,B>,C>;}
pub struct PairRecFn<T1,T2,T3>(T1,T2,T3);
impl<T1:Type,T2:Type,T3:Type> Type for PairRecFn<T1,T2,T3> {type T=Arrow2<Arrow2<T1,T2,T3>,Product<T1,T2>,T3>;}
impl<T1:Type,T2:Type,T3:Type,A:Type<T=T1>,B:Type<T=T2>,F:Type<T=Arrow2<T1,T2,T3>>>
Func2<F,Pair<A,B>> for PairRecFn<T1,T2,T3> where
	F: Func2Type<T1=T1,T2=T2,T3=T3>,
	F: Func2<A,B>,
{type F=<F as Func2<A,B>>::F;}
pub struct PairRec<A,B>(A,B);
impl<A:Type,B:Type> Type for PairRec<A,B> {type T=Pi<Star,PairRecFam<A,B>>;}
impl<A:Type,B:Type,C:Type<T=Star>> DFunc<C> for PairRec<A,B> {type D=PairRecFn<A,B,C>;}
