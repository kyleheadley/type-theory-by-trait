use typed::*;
use arrow::*;
use darrow::*;

/// Type of unit value
pub struct UnitType;
impl Typed for UnitType {type T=Star;}

/// The unit value
pub struct Unit;
impl Typed for Unit {type T=UnitType;}

/// Type of pairs
pub struct Product;
impl Typed for Product {type T=Star;}
//pub trait ProductTyped : Typed<T=Product> {type T1:Typed; type T2:Typed;}

/// Pairs of values
pub struct Pair<A:Typed,B:Typed>(A,B);
impl<A:Typed,B:Typed> Typed for Pair<A,B> {type T=Product;}
//impl<T1:Typed,T2:Typed,A:Typed<T=T1>,B:Typed<T=T2>> ProductTyped for Pair<A,B> {type T1=T1; type T2=T2;}


/// Projection of first value (parameterized by value type)
pub struct Fst<T1:Typed,T2:Typed>(T1,T2);
impl<T1:Typed,T2:Typed> Typed for Fst<T1,T2> {type T=Arrow<Product,T1>;}
impl<T1:Typed,T2:Typed,A:Typed<T=T1>,B:Typed<T=T2>> Func<Pair<A,B>> for Fst<T1,T2> {type F=A;}

/// Dependent first projection
pub struct Dfst;
impl Typed for Dfst{type T=Pi<Product,DfstFm>;}
impl<T1:Typed<T=Star>,A:Typed<T=T1>,B:Typed> DFunc<Pair<A,B>> for Dfst {type D=A;}
pub struct DfstFm;
impl Typed for DfstFm{type T=Arrow<Product,Star>;}
impl<T1:Typed<T=Star>,A:Typed<T=T1>,B:Typed> Func<Pair<A,B>> for DfstFm {type F=T1;}
//impl<T1:Typed<T=Star>,P:ProductTyped<T1=T1>> Func<P> for DfstFm {type F=T1;}

// /// Projection of second value (parameterized by value type)
// pub struct Snd<T1:Typed,T2:Typed>(T1,T2);
// impl<T1:Typed,T2:Typed> Typed for Snd<T1,T2> {type T=Arrow<Product<T1,T2>,T2>;}
// impl<T1:Typed,T2:Typed,A:Typed<T=T1>,B:Typed<T=T2>> Func<Pair<A,B>> for Snd<T1,T2> {type F=B;}

// // trial implementation
// /// PairRec<A,B>:Π(C:U)[(A->B->C)->(AxB)->C]
// pub struct PairRecFam<A,B>(A,B);
// impl<A:Typed,B:Typed> Typed for PairRecFam<A,B> {type T=Arrow<Star,Star>;}
// impl<A:Typed,B:Typed,C:Typed<T=Star>> Func<C> for PairRecFam<A,B> {type F=Arrow2<Arrow2<A,B,C>,Product<A,B>,C>;}
// pub struct PairRecFn<T1,T2,T3>(T1,T2,T3);
// impl<T1:Typed,T2:Typed,T3:Typed> Typed for PairRecFn<T1,T2,T3> {type T=Arrow2<Arrow2<T1,T2,T3>,Product<T1,T2>,T3>;}
// impl<T1:Typed,T2:Typed,T3:Typed,A:Typed<T=T1>,B:Typed<T=T2>,F:Typed<T=Arrow2<T1,T2,T3>>>
// Func2<F,Pair<A,B>> for PairRecFn<T1,T2,T3> where
// 	F: AbsArrow2<T1=T1,T2=T2,T3=T3>,
// 	F: Func2<A,B>,
// {type F=<F as Func2<A,B>>::F;}
// pub struct PairRec<A,B>(A,B);
// impl<A:Typed,B:Typed> Typed for PairRec<A,B> {type T=Pi<Star,PairRecFam<A,B>>;}
// impl<A:Typed,B:Typed,C:Typed<T=Star>> DFunc<C> for PairRec<A,B> {type D=PairRecFn<A,B,C>;}

