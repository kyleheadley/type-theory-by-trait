use base::*;

pub trait Bool {}
impl<B:Type<Bool>> Bool for B {}

pub struct True;
impl Type<Bool> for True {}

pub struct False;
impl Type<Bool> for False {}

pub struct Or;
impl Func2<Bool,Bool,Bool,True,True> for Or {type F=True;}
impl Func2<Bool,Bool,Bool,True,False> for Or {type F=True;}
impl Func2<Bool,Bool,Bool,False,True> for Or {type F=True;}
impl Func2<Bool,Bool,Bool,False,False> for Or {type F=False;}

pub trait OrFn<B1:Bool,B2:Bool> {type F:Bool;}
pub struct TypedOr;
impl OrFn<True,True> for TypedOr {type F=True;}
impl OrFn<True,False> for TypedOr {type F=True;}
impl OrFn<False,True> for TypedOr {type F=True;}
impl OrFn<False,False> for TypedOr {type F=False;}

