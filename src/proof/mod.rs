// Meta-language natural number
trait Counter {}
struct Base;
struct Next<C:Counter>(C);
impl Counter for Base {}
impl<C:Counter> Counter for Next<C> {}
// Meta-language functions over counters
trait CounterFn1<C0:Counter> { type C:Counter; }
trait CounterFn2<C0:Counter,C1:Counter> { type C:Counter; }
trait UnEqCounterFn2<C0:Counter,C1:Counter> { type C:Counter; }
//impl<C0:Counter,C1:Counter,C:UnEqCounterFn2<C0,C1>> CounterFn2<C0,C1> for C { type C = C::C; }
// Predecessor function
struct Pred;
impl<I:Counter> CounterFn1<Next<I>> for Pred { type C = I; }
// Return the greater of the two counters
struct Max;
// we'd like to use this, but how then would you define the fn?
// impl<C:Counter> CounterFn2<C,C> for Max { type C = C; }
impl<I2:Counter> CounterFn2<Base,I2> for Max { type C = I2; }
impl<I1:Counter> CounterFn2<Next<I1>,Base> for Max { type C = Next<I1>; }
impl<I1:Counter,I2:Counter> CounterFn2<Next<I1>,Next<I2>> for Max where
	Max : CounterFn2<I1,I2>
{ type C = Next<<Max as CounterFn2<I1,I2>>::C>; }

// Equality Bounds (overflows on use)
trait NotEqual {}
impl<C:Counter> NotEqual for (Base,Next<C>) {}
impl<C:Counter> NotEqual for (Next<C>,Base) {}
impl<C1:Counter,C2:Counter> NotEqual for (Next<C1>,Next<C2>) where (C1,C2) : NotEqual {}
trait Equal {}
impl<C:Counter> Equal for (C,C) {}

// Type and universe level of an object
trait Typed {
	type U : Counter;
	type T : Typed<U=Next<Self::U>>;
}
// Simplified type bound, avoid early well-formedness check
// (Accessor trait)
trait Typ { type T; }
impl<A:Typed> Typ for A { type T = A::T; } 

// Generic Type for objects in Universe O
struct Type<O:Counter>(O);
impl<O:Counter> Typed for Type<O> {
	type U = Next<O>;
	type T = Type<Next<O>>;
}

// Abstract bound for types
trait AbstractType {type U: Counter;}
impl<U:Counter> AbstractType for Type<U> {type U = U;}

// Universe of small types
type UTypes = Next<Base>;
// Type of small types
type TTypes = Type<Next<Base>>;

// Simple(non-dependent) function type
struct Arrow<D:Typed,C:Typed>(D,C);
impl<D:Typed,C:Typed> Typed for Arrow<D,C> where
	Max : CounterFn2<D::U,C::U>,
{
	type U = <Max as CounterFn2<D::U,C::U>>::C;
	type T = Type<Self::U>;
}

// Abstract bound for arrows
trait AbstractArrow {type Dom:Typed; type CoD:Typed;}
impl<D:Typed,C:Typed> AbstractArrow for Arrow<D,C> {
	type Dom=D; type CoD=C;
}

// Allows defining a function
trait Func<D:Typed> : Typed where
	Self::T : AbstractArrow<Dom=D::T>,
{
	type F : Typ<T=<Self::T as AbstractArrow>::CoD>;
}

// Pi/Dependent function types Π(a:α)β a,
//	where a is a parameter and ∃u.β:α->Type<u>
struct DArrow<D:Typed,F:Typed>(D,F) where
	F::T : AbstractArrow<Dom=D>,
	<F::T as AbstractArrow>::CoD : AbstractType
;
impl<D:Typed,F:Typed> Typed for DArrow<D,F> where
	F::T : AbstractArrow<Dom=D>,
	<F::T as AbstractArrow>::CoD : AbstractType,
	Max : CounterFn2<D::U,F::U>,
{
	type U = <Max as CounterFn2<D::U,F::U>>::C;
	type T = Type<Self::U>;
}

// Abstract bound for dependent arrows
trait AbstractDArrow { type Dom:Typed; type Fam:Typed; }
impl<D:Typed,F:Typed> AbstractDArrow for DArrow<D,F> where
	F::T : AbstractArrow<Dom=D>,
	<F::T as AbstractArrow>::CoD : AbstractType,
{ type Dom=D; type Fam=F; }

// Allows defining a Dependent function
trait DFunc<D:Typed> : Typed where
	Self::T : AbstractDArrow<Dom=D::T>,
	<<Self::T as AbstractDArrow>::Fam as Typed>::T
		: AbstractArrow<Dom=<Self::T as AbstractDArrow>::Dom>,
	<Self::T as AbstractDArrow>::Fam : Func<D>,
{ type D : Typ<T=<<Self::T as AbstractDArrow>::Fam as Func<D>>::F>; }

// Type of axioms to be proven
struct Proposition<ProofUniverse:Counter>(ProofUniverse);
impl<U:Counter> Typed for Proposition<U> {
	type U = Next<Next<U>>;
	type T = Type<Self::U>;
}
type Prop = Proposition<Base>;

// Equality Proposition, A = B
struct Eq<A,B>(A,B);
impl<P,T,A,B> Typed for Eq<A,B> where
	P : Counter, // Proof Universe number
	T : Typed<U=Next<Next<P>>>, // Type of A and B
	A : Typed<U=Next<P>,T=T>,
	B : Typed<U=Next<P>,T=T>,
{
	type U = Next<P>;
	type T = Proposition<P>;
}

// Proof: a =A a 
struct Refl<A>(A);
impl<U,A> Typed for Refl<A> where
	U : Counter,
	A: Typed<U=Next<U>>,
{
	type U = U;
	type T = Eq<A,A>;
}
// // Proof: a =T b
// struct EqAxiom<A,B>(A,B);
// impl<U,T,A,B> Typed for EqAxiom<A,B> where
// 	U : Counter,
// 	T : Typed<U=Next<U>>,
// 	A : Typed<U=U,T=T>,
// 	B : Typed<U=U,T=T>,
// {
// 	type U = U;
// 	type T = Eq<A::T,B::T>;
// }

// Proof: A = B -> B = A
struct Symm<A,B>(A,B);
impl<P,T,A,B> Typed for Symm<A,B> where
	P : Counter, // Proof Universe number
	T : Typed<U=Next<Next<P>>>, // Type of A and B
	A : Typed<U=Next<P>,T=T>,
	B : Typed<U=Next<P>,T=T>,
	// need one or the other of the following
	Max : CounterFn2<P,P,C=P>,
	//Arrow<Eq<A,B>,Eq<B,A>> : Typed<U=Next<P>>,
{
	type U = P;
	type T = Arrow<Eq<A,B>,Eq<B,A>>;
}
impl<U:Counter,A:Typed<U=Next<U>>> Func<Refl<A>> for Symm<A,A> where
	Max : CounterFn2<U,U,C=U>,
{ type F = Refl<A>; }
// impl<U,T,A,B> Func<EqAxiom<A,B>> for Symm<A,B> where
// 	U : Counter,
// 	T : Typed<U=Next<Next<U>>>,
// 	A : Typed<U=Next<U>,T=T>,
// 	B : Typed<U=Next<U>,T=T>,
// 	Max : CounterFn2<U,U,C=U>,
// { type F = EqAxiom<B,A>; }

// TODO: trans

// Type of natural numbers
struct Nat;
impl Typed for Nat { type U=UTypes; type T=TTypes; }

// The natural numbers
struct Z;
struct S<N:Typ<T=Nat>>(N);
impl Typed for Z { type U=Base; type T=Nat; }
impl<N:Typ<T=Nat>> Typed for S<N> {
	type U=Base;
	type T=Nat;
}

// Type of Boolean values
struct Bool;
impl Typed for Bool { type U=UTypes; type T=TTypes; }

// The booleans
struct False;
struct True;
impl Typed for False { type U=Base; type T=Bool; }
impl Typed for True { type U=Base; type T=Bool; }

// Axiom Bool = Nat
struct BoolIsNat;
impl Typed for BoolIsNat {
	type U = Base;
	type T = Eq<Bool,Nat>;
}

fn checktype<T,O:Typ<T=T>>() {}
fn main() {
	checktype::<
		Arrow<Eq<Nat,Bool>,Eq<Bool,Nat>>,
		Symm<Nat,Bool>
	>();
	checktype::<
		Eq<Bool,Bool>,
		<Symm<Bool,Bool> as Func<Refl<Bool>>>::F
	>();
}










