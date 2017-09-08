/// Property that represents a typing of the object its implemented for
pub trait Type {type T;}
/// Temporary type-of-types
pub struct Star;

/// Untyped judgment on one variable (predicate)
pub trait Judge<A> {}
/// Untyped judgment on two variables
pub trait Judge2<A,B> {}
