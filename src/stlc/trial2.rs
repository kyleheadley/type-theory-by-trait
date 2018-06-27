// Meta values and functions
// -------------------------

// Bool values
struct False;
struct True;

// Option values
struct None;
struct Some<V>(V);

// Nat values
struct Zero;
struct Succ<N>(N);

// Nat equality function
trait NatEq<N> { type Eq; }
impl NatEq<Zero> for Zero { type Eq=True; }
impl<N> NatEq<Succ<N>> for Zero { type Eq=False; }
impl<N> NatEq<Zero> for Succ<N> { type Eq=False; }
impl<N1,N2,E> NatEq<Succ<N1>> for Succ<N2> where
	N2: NatEq<N1,Eq=E>
{ type Eq=E; }

// Context
struct EmptyCtx;
struct TypeCtx<Id,Typ,Next>(Id,Typ,Next);

// context membership function
trait Contains<Id> { type Result; }
impl<N> Contains<N> for EmptyCtx
{ type Result=None; }

impl<Check,First,Typ,Next,Eq,R>
Contains<Check> for TypeCtx<First,Typ,Next> where
	Check: NatEq<First,Eq=Eq>,
	Next: Contains2<Eq,Typ,Check,Result=R>,
{ type Result=R; }

trait Contains2<Eq,Map,Check> { type Result; }

impl<Map,C,Cxt> Contains2<True,Map,C> for Cxt
{ type Result=Some<Map>; }

impl<Map,C> Contains2<False,Map,C> for EmptyCtx
{ type Result=None; }

impl<Check,First,T,Typ,Next,Eq,R>
Contains2<False,T,Check> for TypeCtx<First,Typ,Next> where
	Check: NatEq<First,Eq=Eq>,
	Next: Contains2<Eq,Typ,Check,Result=R>,
{ type Result=R; }

// Syntax
// ------

trait WFType {}

struct Number;
impl WFType for Number {}

// we use our meta values in Num and Var below
trait WFNat {}
impl WFNat for Zero {}
impl<N:WFNat> WFNat for Succ<N> {}

struct Arrow<T1,T2>(T1,T2);
impl<T1:WFType,T2:WFType> WFType for Arrow<T1,T2> {}

trait Expr {}

struct Num<N>(N);
impl<N:WFNat> Expr for Num<N> {}

struct Plus<N1,N2>(N1,N2);
impl<N1:Expr,N2:Expr> Expr for Plus<N1,N2> {}

struct Var<N>(N);
impl<N:WFNat> Expr for Var<N> {}

struct Lam<V,T,E>(V,T,E);
impl<N:WFNat,T:WFType,E:Expr> Expr for Lam<Var<N>,T,E> {}

struct App<E1,E2>(E1,E2);
impl<E1:Expr,E2:Expr> Expr for App<E1,E2> {}

// Statics
// -------

trait Typed<Ctx> { type T; }

impl<N,Ctx> Typed<Ctx> for Num<N> { type T=Number; }

impl<N1,N2,Ctx> Typed<Ctx> for Plus<N1,N2> where
	N1:Typed<Ctx,T=Number>,
	N2:Typed<Ctx,T=Number>,
{ type T=Number; }

impl<N,Ctx,T> Typed<Ctx> for Var<N> where
	Ctx:Contains<N,Result=Some<T>>
{ type T=T; }

impl<Ctx,N,T1,T2,E> Typed<Ctx> for Lam<Var<N>,T1,E> where
	E:Typed<TypeCtx<N,T1,Ctx>,T=T2>,
{ type T=Arrow<T1,T2>; }

impl<Ctx,E1,E2,T1,T2> Typed<Ctx> for App<E1,E2> where
	E1:Typed<Ctx,T=Arrow<T1,T2>>,
	E2:Typed<Ctx,T=T1>
{ type T=T2; }

// well-formedness checks
fn is_wf_expr<E:Expr>(e:&E) {}
fn is_wf_type<T:WFType>(t:&T) {}
fn is_typed<E,T>(e:&E,t:&T) where
	E:Typed<EmptyCtx,T=T>
{}

// Parsing
// -------
macro_rules! run {
	($($x:tt)+) => (expr![$($x)*].code());
}
macro_rules! expr {
	(($($e:tt)+)) => (expr![$($e)+]);
	({^$e:expr}) => ($e);
	(0) => (Num(Zero));
	(1) => (Num(Succ(Zero)));
	(2) => (Num(Succ(Succ(Zero))));
	(3) => (Num(Succ(Succ(Succ(Zero)))));
	(4) => (Num(Succ(Succ(Succ(Succ(Zero))))));
	(5) => (Num(Succ(Succ(Succ(Succ(Succ(Zero)))))));
	(6) => (Num(Succ(Succ(Succ(Succ(Succ(Succ(Zero))))))));
	(7) => (Num(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Zero)))))))));
	(8) => (Num(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Zero))))))))));
	(9) => (Num(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Zero)))))))))));
	(a) => (Var(Zero));
	(b) => (Var(Succ(Zero)));
	(c) => (Var(Succ(Succ(Zero))));
	(d) => (Var(Succ(Succ(Succ(Zero)))));
	(e) => (Var(Succ(Succ(Succ(Succ(Zero))))));
	(v) => (Var(Succ(Succ(Succ(Succ(Succ(Zero)))))));
	(w) => (Var(Succ(Succ(Succ(Succ(Succ(Succ(Zero))))))));
	(x) => (Var(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Zero)))))))));
	(y) => (Var(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Zero))))))))));
	(z) => (Var(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Succ(Zero)))))))))));
	(lam ($x:ident : $($t:tt)+)$(($($ts:tt)+))+ $($e:tt)+) => (Lam(
		expr![$x],
		typ![$($t)+],
		expr![lam $(($($ts)+))+ $($e)+]
	));
	(lam ($x:ident : $($t:tt)+) $($e:tt)+) => (Lam(
		expr![$x],
		typ![$($t)+],
		expr![$($e)+]
	));
	($n:tt + $($ns:tt)+) => (Plus(expr![$n],expr![$($ns)+]));
	($a:tt $p:tt) => (App(expr![$a],expr![$p]));
	($a:tt $p:tt $($ps:tt)+) => (expr![{^App(expr![$a],expr![$p])} $($ps),+]);
}
macro_rules! typ {
	(($($ts:tt)+)) => (typ![$ts]);
	(N) => (Number);
	($t:tt -> $($ts:tt)+) => (Arrow(typ![$t],typ![$($ts)+]));
}

// Examples
// --------

fn test(){
	struct Red;
	let e = Plus(Num(Red),Num(Red));
	let t = Number;
	// is_wf_expr(&e); // fails
	is_wf_type(&t);
	is_typed(&e,&t); // typing Num doesn't rely on the actual nat

	// λx.x
	// let e = Lam(Var(Zero),Number,Var(Zero));
	// let t = Arrow(Number,Number);
	let e = expr![lam (x : N) x];
	let t = typ![N -> N];
	is_wf_expr(&e);
	is_wf_type(&t);
	is_typed(&e,&t);

	// (λy.λx.x+y) 1 2
	// let e = App(App(
	// 	Lam(Var(Succ(Zero)),Number,
	// 		Lam(Var(Zero),Number,
	// 			Plus(Var(Zero),Var(Succ(Zero)))
	// 		)
	// 	), Num(Succ(Zero))
	// ), Num(Succ(Succ(Zero))));
	// let t = Number;
	let e = expr![(lam (y : N)(x : N) x+y) 1 2];
	let t = typ![N];
	is_wf_expr(&e);
	is_wf_type(&t);
	is_typed(&e,&t);

	// (λy.λx.x y) 2 (λx.1+x)
	// let e = App(App(
	// 	Lam(Var(Succ(Zero)),Number,
	// 		Lam(Var(Zero),Arrow(Number,Number),
	// 			App(Var(Zero),Var(Succ(Zero)))
	// 		)
	// 	), Num(Succ(Succ(Zero)))
	// ), Lam(Var(Zero),Number,Plus(Num(Succ(Zero)),Var(Zero))));
	// let t = Number;
	let e = expr![(lam (y:N)(x:N->N) x y) 2 (lam (x:N) 1+x)];
	let t = typ![N];
	is_wf_expr(&e);
	is_wf_type(&t);
	is_typed(&e,&t);
}
