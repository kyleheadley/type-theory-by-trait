/// I don't know how useful this will be
/// It could be used with a macro to guarentee all varients are used
/// It doesn't work with constructors, it needs functions
/// More experimenting is needed


struct Gen;
trait Func0 {type Out;}
trait Func1<I> {type Out;}

trait Bool {
	type True;
	type False;
}

struct True;
struct False;

impl Bool for Gen {
	type True = True;
	type False = False;
}

struct Not;
impl Bool for Not {
	type True = <Gen as Bool>::False;
	type False = <Gen as Bool>::True;
}
impl Func1<<Gen as Bool>::False> for Not {
	type Out = <Not as Bool>::False;
}
impl Func1<<Gen as Bool>::True> for Not {
	type Out = <Not as Bool>::True;
}

type T = <Gen as Bool>::True;
type F = <Gen as Bool>::False;
type NT = <Not as Func1<T>>::Out;


