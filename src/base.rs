/// Property that represents a typing of the object it's implemented for
///
/// This is required of all values manipulated in this computation system
pub trait Type {type T;}

/// Type-of-types
pub struct Star;
impl Type for Star {type T=StarType;}

/// Type of Stars, not avaliable for manipulation
pub struct StarType;

/// Untyped judgment on one variable (predicate)
pub trait Judge<A> {}
/// Untyped judgment on two variables
pub trait Judge2<A,B> {}
