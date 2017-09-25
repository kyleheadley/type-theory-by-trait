/// Property that represents a typing of the object it's implemented for
///
/// This is required of all values manipulated in this computation system
pub trait Typed {type T;}

/// Type-of-types
/// * : StarType
pub struct Star;
impl Typed for Star {type T=StarType;}

/// Type of Stars, not avaliable for manipulation
pub struct StarType;

/// Untyped judgment on one variable (predicate)
pub trait Judge<A> {}
/// Untyped judgment on two variables
pub trait Judge2<A,B> {}

/// Untyped Function of one variable
pub trait MetaFunc<A> {type M;}
/// Untyped Function of two variables
pub trait MetaFunc2<A,B> {type M;}
