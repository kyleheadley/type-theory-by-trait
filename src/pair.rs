use typed::*;
use arrow::*;
use darrow::*;

/// Type of unit value
pub struct UnitType;
impl Typed for UnitType {
	fn reflect() -> String {format!("[]")}
	type T=Star;
}

/// The unit value
pub struct Unit;
impl Typed for Unit {
	fn reflect() -> String {format!("()")}
	type T=UnitType;
}

/// Type of pairs
pub struct Product<T1:Typed,T2:Typed>(T1,T2);
impl<A:Typed,B:Typed> Typed for Product<A,B> {
	fn reflect() -> String {format!("({}x{})",A::reflect(),B::reflect())}
	type T=Star;
}

/// Pairs of values
pub struct Pair<A:Typed,B:Typed>(A,B);
impl<T1:Typed,T2:Typed,A:Typed<T=T1>,B:Typed<T=T2>> Typed for Pair<A,B> {
	fn reflect() -> String {format!("({},{})",A::reflect(),B::reflect())}
	type T=Product<T1,T2>;
}

/// Projection of first value (parameterized by value type)
pub struct Fst<T1:Typed,T2:Typed>(T1,T2);
impl<T1:Typed,T2:Typed> Typed for Fst<T1,T2> {
	fn reflect() -> String {format!("fst({}x{})",T1::reflect(),T2::reflect())}
	type T=Arrow<Product<T1,T2>,T1>;
}
impl<T1:Typed,T2:Typed,A:Typed<T=T1>,B:Typed<T=T2>> Func<Pair<A,B>> for Fst<T1,T2> {type F=A;}

/// Projection of second value (parameterized by value type)
pub struct Snd<T1:Typed,T2:Typed>(T1,T2);
impl<T1:Typed,T2:Typed> Typed for Snd<T1,T2> {
	fn reflect() -> String {format!("snd({}x{})",T1::reflect(),T2::reflect())}
	type T=Arrow<Product<T1,T2>,T2>;
}
impl<T1:Typed,T2:Typed,A:Typed<T=T1>,B:Typed<T=T2>> Func<Pair<A,B>> for Snd<T1,T2> {type F=B;}

// trial implementation
/// PairRec<A,B>:Π(C:U)[(A->B->C)->(AxB)->C]
pub struct PairRecFam<A,B>(A,B);
impl<A:Typed,B:Typed> Typed for PairRecFam<A,B> {
	fn reflect() -> String {format!("PairRecFm")}
	type T=Arrow<Star,Star>;
}
impl<A:Typed,B:Typed,C:Typed<T=Star>> Func<C> for PairRecFam<A,B> {type F=Arrow2<Arrow2<A,B,C>,Product<A,B>,C>;}
pub struct PairRecFn<T1,T2,T3>(T1,T2,T3);
impl<T1:Typed,T2:Typed,T3:Typed> Typed for PairRecFn<T1,T2,T3> {
	fn reflect() -> String {format!("PairRecFn")}
	type T=Arrow2<Arrow2<T1,T2,T3>,Product<T1,T2>,T3>;
}
impl<T1:Typed,T2:Typed,T3:Typed,A:Typed<T=T1>,B:Typed<T=T2>,F:Typed<T=Arrow2<T1,T2,T3>>>
Func2<F,Pair<A,B>> for PairRecFn<T1,T2,T3> where
	F: AbsArrow2<T1=T1,T2=T2,T3=T3>,
	F: Func2<A,B>,
{type F=<F as Func2<A,B>>::F;}
pub struct PairRec<A,B>(A,B);
impl<A:Typed,B:Typed> Typed for PairRec<A,B> {
	fn reflect() -> String {format!("PairRec")}
	type T=Pi<Star,PairRecFam<A,B>>;
}
impl<A:Typed,B:Typed,C:Typed<T=Star>> DFunc<C> for PairRec<A,B> {type D=PairRecFn<A,B,C>;}

