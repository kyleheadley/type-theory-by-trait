// Meta-language natural number
trait Counter {}
struct Base;
struct Next<C:Counter>(C);
impl Counter for Base {}
impl<C:Counter> Counter for Next<C> {}
// Meta-language functions over counters
trait CounterFn1<C0:Counter> { type C:Counter; }
trait CounterFn2<C0:Counter,C1:Counter> { type C:Counter; }
// Predecessor function
struct Pred;
impl<I:Counter> CounterFn1<Next<I>> for Pred { type C = I; }
// Return the greater of the two counters
struct Max;
impl<I2:Counter> CounterFn2<Base,I2> for Max { type C = I2; }
impl<I1:Counter> CounterFn2<Next<I1>,Base> for Max { type C = Next<I1>; }
impl<I1:Counter,I2:Counter> CounterFn2<Next<I1>,Next<I2>> for Max where
	Max : CounterFn2<I1,I2>
{ type C = Next<<Max as CounterFn2<I1,I2>>::C>;}

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
trait AbstractArrow {type Dom; type CoD;}
impl<I:Typed,O:Typed> AbstractArrow for Arrow<I,O> {
	type Dom=I; type CoD=O;
}

// Pi/Dependent function types Π(a:α)β a,
//	where a is a parameter and ∃u.β:α->Type<u>
struct DArrow<D:Typed,C:Typed>(D,C) where
	C::T : AbstractArrow<Dom=D>,
	<C::T as AbstractArrow>::CoD : AbstractType
;
impl<D:Typed,C:Typed> Typed for DArrow<D,C> where
	C::T : AbstractArrow<Dom=D>,
	<C::T as AbstractArrow>::CoD : AbstractType,
	Max : CounterFn2<D::U,C::U>,
{
	type U = <Max as CounterFn2<D::U,C::U>>::C;
	type T = Type<Self::U>;
}

// Abstract bound for dependent arrows
trait AbstractDArrow {type Dom; type CoD; }
impl<I:Typed,O:Typed> AbstractDArrow for DArrow<I,O> where
	O::T : AbstractArrow<Dom=I>,
	<O::T as AbstractArrow>::CoD : AbstractType,
{ type Dom=I; type CoD=O; }

// Type of axioms to be proven
struct Proposition<ProofUniverse:Counter>(ProofUniverse);
impl<U:Counter> Typed for Proposition<U> {
	type U = Next<Next<U>>;
	type T = Type<Self::U>;
}
type Prop = Proposition<Base>;

// Equality Type
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

// The element of equality type
struct Refl<A>(A);
impl<U,A> Typed for Refl<A> where
	U : Counter,
	A: Typed<U=Next<U>>,
{
	type U = U;
	type T = Eq<A,A>;
}

// TODO: symm, trans

// Type of natural numbers
struct Nat;
impl Typed for Nat { type U=UTypes; type T=TTypes; }

// The natural numbers
struct Z;
struct S<N:Typ<T=Nat>>(N);
impl Typed for Z { type U=Base; type T = Nat;}
impl<N:Typ<T=Nat>> Typed for S<N> {
	type U=Base;
	type T=Nat;
}












