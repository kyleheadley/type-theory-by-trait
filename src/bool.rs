use base::*;

pub trait Bool {}
impl BaseType for Bool {}

pub struct True;
impl Bool for True {}

pub struct False;
impl Bool for False {}

pub struct Or;
impl Func2<True,True> for Or {type F=True;}
impl Func2<True,False> for Or {type F=True;}
impl Func2<False,True> for Or {type F=True;}
impl Func2<False,False> for Or {type F=False;}

pub trait OrFn<B1:Bool,B2:Bool> {type F:Bool;}
pub struct TypedOr;
impl OrFn<True,True> for TypedOr {type F=True;}
impl OrFn<True,False> for TypedOr {type F=True;}
impl OrFn<False,True> for TypedOr {type F=True;}
impl OrFn<False,False> for TypedOr {type F=False;}

