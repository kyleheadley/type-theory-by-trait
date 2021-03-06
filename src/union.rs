use typed::*;
use arrow::*;
use darrow::*;

/// Type of empty value
pub struct EmptyType;
impl Typed for EmptyType {
	fn reflect() -> String {format!("BOT")}
	type T=Star;
}

/// The empty value (no public constructor!)
struct Empty;
impl Typed for Empty {
	fn reflect() -> String {format!("bot")}
	type T=EmptyType;
}

/// Type of unions
pub struct Coproduct<T1:Typed,T2:Typed>(T1,T2);
impl<A:Typed,B:Typed> Typed for Coproduct<A,B> {
	fn reflect() -> String {format!("({}+{})",A::reflect(),B::reflect())}
	type T=Star;
}

/// Constructor for unions of left type
pub struct Inl<T1,T2,A>(T1,T2,A);
impl<T1:Typed,T2:Typed,A:Typed<T=T1>> Typed for Inl<T1,T2,A> {
	fn reflect() -> String {format!("inl({})",A::reflect())}
	type T=Coproduct<T1,T2>;
}
/// Constructor for unions of right type
pub struct Inr<T1,T2,B>(T1,T2,B);
impl<T1:Typed,T2:Typed,B:Typed<T=T2>> Typed for Inr<T1,T2,B> {
	fn reflect() -> String {format!("inr({})",B::reflect())}
	type T=Coproduct<T1,T2>;
}

// trial implementation
/// (Curried) recursor for coproduct types 
pub struct CoproductRec<A:Typed,B:Typed>(A,B);
impl<A:Typed,B:Typed> Typed for CoproductRec<A,B> {
	fn reflect() -> String {format!("CopRec")}
	type T=Pi<Star,CoproductRecFm<A,B>>;
}
impl<A:Typed,B:Typed,C:Typed<T=Star>> DFunc<C> for CoproductRec<A,B> {type D=CoproductRecInner0<A,B,C>;}
pub struct CoproductRecFm<A:Typed,B:Typed>(A,B);
impl<A:Typed,B:Typed> Typed for CoproductRecFm<A,B> {
	fn reflect() -> String {format!("CopRecFm")}
	type T=Arrow<Star,Star>;
}
impl<A:Typed,B:Typed,C:Typed<T=Star>> Func<C> for CoproductRecFm<A,B>
{type F=Arrow<Arrow<A,C>,Arrow<Arrow<B,C>,Arrow<Coproduct<A,B>,C>>>;}
pub struct CoproductRecInner0<A:Typed,B:Typed,C:Typed>(A,B,C);
impl<A:Typed,B:Typed,C:Typed> Typed for CoproductRecInner0<A,B,C> {
	fn reflect() -> String {format!("CR0")}
	type T=Arrow<Arrow<A,C>,Arrow<Arrow<B,C>,Arrow<Coproduct<A,B>,C>>>;
}
impl<A:Typed,B:Typed,C:Typed,G0:Typed<T=Arrow<A,C>>> Func<G0> for CoproductRecInner0<A,B,C> {type F=CoproductRecInner1<A,B,C,G0>;}
pub struct CoproductRecInner1<A:Typed,B:Typed,C:Typed,G0:Typed<T=Arrow<A,C>>>(A,B,C,G0);
impl<A:Typed,B:Typed,C:Typed,G0:Typed<T=Arrow<A,C>>> Typed for CoproductRecInner1<A,B,C,G0> {
	fn reflect() -> String {format!("CR1")}
	type T=Arrow<Arrow<B,C>,Arrow<Coproduct<A,B>,C>>;
}
impl<A:Typed,B:Typed,C:Typed,G0:Typed<T=Arrow<A,C>>,G1:Typed<T=Arrow<B,C>>> Func<G1> for CoproductRecInner1<A,B,C,G0> {type F=CoproductRecInner2<A,B,C,G0,G1>;}
pub struct CoproductRecInner2<A:Typed,B:Typed,C:Typed,G0:Typed<T=Arrow<A,C>>,G1:Typed<T=Arrow<B,C>>>(A,B,C,G0,G1);
impl<A:Typed,B:Typed,C:Typed,G0:Typed<T=Arrow<A,C>>,G1:Typed<T=Arrow<B,C>>> Typed for CoproductRecInner2<A,B,C,G0,G1> {
	fn reflect() -> String {format!("CR2")}
	type T=Arrow<Coproduct<A,B>,C>;
}
impl<A:Typed,B:Typed,C:Typed,G0:Typed<T=Arrow<A,C>>,G1:Typed<T=Arrow<B,C>>,X:Typed<T=A>> Func<Inl<A,B,X>> for CoproductRecInner2<A,B,C,G0,G1> where
	G0: Func<X>+AbsArrow<T1=A,T2=C>,
{type F=<G0 as Func<X>>::F;}
impl<A:Typed,B:Typed,C:Typed,G0:Typed<T=Arrow<A,C>>,G1:Typed<T=Arrow<B,C>>,X:Typed<T=B>> Func<Inr<A,B,X>> for CoproductRecInner2<A,B,C,G0,G1> where
	G1: Func<X>+AbsArrow<T1=B,T2=C>,
{type F=<G1 as Func<X>>::F;}
