trait Type {}

// Unit type
struct Unit;
impl Type for Unit {}

// Bool type
trait WFBool {}
struct Bool;
impl Type for Bool {}
struct True;
impl WFBool for True {}
struct False;
impl WFBool for False {}

// Nat type
trait WFNat {}
struct Nat;
impl Type for Nat {}
struct Zero;
impl WFNat for Zero {}
struct Succ<N:WFNat>(N);
impl<N:WFNat> WFNat for Succ<N> {}

// Nat equality function
trait NatEq<N:WFNat> { type E:WFBool; }
impl NatEq<Zero> for Zero { type E=True; }
impl<N:WFNat> NatEq<Succ<N>> for Zero { type E=False; }
impl<N:WFNat> NatEq<Zero> for Succ<N> { type E=False; }
impl<N1:WFNat,N2:WFNat> NatEq<Succ<N1>> for Succ<N2> where
	N2: NatEq<N1>
{ type E=<N2 as NatEq<N1>>::E; }

// Arrow type
trait WFArrow { type In:Type; type Out:Type; }
struct Arrow<T1,T2>(T1,T2);
impl<T1:Type,T2:Type> Type for Arrow<T1,T2> {}

// Contexts
trait Context {}
struct Empty;
impl Context for Empty {}
struct Cxt<V,T,C:Context>(V,T,C);
impl<N:WFNat,T:Type,C:Context> Context for Cxt<Var<N>,T,C> {}

// context membership function
trait Contains<V> { type C:WFBool; type T:Type; }
impl<N:WFNat> Contains<Var<N>> for Empty { type C=False; type T=Unit; }
impl<N,N1,T,C> Contains<Var<N>> for Cxt<Var<N1>,T,C> where
	N:WFNat,N1:WFNat,T:Type,C:Context,
	N1: NatEq<N>,
	C: Contains2<<N1 as NatEq<N>>::E,T,Var<N>>,
{ 
	type C=< C as Contains2<< N1 as NatEq<N> >::E,T,Var<N>> >::C;
	type T=< C as Contains2<< N1 as NatEq<N> >::E,T,Var<N>> >::T;
}
trait Contains2<E,T,V> { type C:WFBool; type T:Type; }
impl<T:Type,V,C:Context> Contains2<True,T,V> for C { type C=True; type T=T; }
impl<T:Type,V> Contains2<False,T,V> for Empty { type C=False; type T=Unit; }
impl<N,N1,T,T1,C1> Contains2<False,T,Var<N>> for Cxt<Var<N1>,T1,C1> where
	N:WFNat,N1:WFNat,T:Type,T1:Type,C1:Context,
	N1: NatEq<N>,
	C1: Contains2<<N1 as NatEq<N>>::E,T,Var<N>>,
{ 
	type C=<C1 as Contains2<<N1 as NatEq<N>>::E,T,Var<N>>>::C;
	type T=<C1 as Contains2<<N1 as NatEq<N>>::E,T,Var<N>>>::T;
}

// Syntax
trait Expr {}

struct Num<N>(N);
impl<N:WFNat> Expr for Num<N> {}

struct Plus<N1,N2>(N1,N2);
impl<N1:Expr,N2:Expr> Expr for Plus<N1,N2> {}

struct Var<N>(N);
impl<N:WFNat> Expr for Var<N> {}

struct Lam<V,T,E>(V,T,E);
impl<N:WFNat,T:Type,E:Expr> Expr for Lam<Var<N>,T,E> {}

struct App<E1,E2>(E1,E2);
impl<E1:Expr,E2:Expr> Expr for App<E1,E2> {}

// Statics
trait Typed<C:Context> { type T:Type; }
impl<N:WFNat,C:Context> Typed<C> for Num<N> { type T=Nat; }
impl<N1,N2,C> Typed<C> for Plus<N1,N2> where
	N1:Typed<C,T=Nat>,N2:Typed<C,T=Nat>,C:Context,
{ type T=Nat; }
impl<N,C,T> Typed<C> for Var<N> where
	N:WFNat,C:Context,T:Type,
	C:Contains<Var<N>,C=True,T=T>
{ type T=T; }
impl<C,N,T1,T2,E> Typed<C> for Lam<Var<N>,T1,E> where 
	C:Context,N:WFNat,T1:Type,T2:Type,E:Expr,
	E:Typed<Cxt<Var<N>,T1,C>,T=T2>,
{ type T=Arrow<T1,T2>; }
impl<C,E1,E2,T1,T2> Typed<C> for App<E1,E2> where
	C:Context,E1:Expr,E2:Expr,T1:Type,T2:Type,
	E1:Typed<C,T=Arrow<T1,T2>>,
	E2:Typed<C,T=T1>
{ type T=T2; }

fn test(){
	fn is_typed<E:Expr,T:Type>(e:E,t:T) where
		E:Typed<Empty,T=T>
	{}

	// λx.x
	let e = Lam(Var(Zero),Nat,Var(Zero));
	let t = Arrow(Nat,Nat);
	is_typed(e,t);
	// (λx.λy.y+x) 1 2
	let e = App(App(
		Lam(Var(Succ(Zero)),Nat,
			Lam(Var(Zero),Nat,
				Plus(Var(Zero),Var(Succ(Zero)))
			)
		), Num(Succ(Zero))
	), Num(Succ(Succ(Zero))));
	let t = Nat;
	is_typed(e,t);
}
