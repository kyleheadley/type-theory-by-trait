use base::*;

pub struct Bool(());

pub struct True;
impl Type<Bool> for True {}

pub struct False;
impl Type<Bool> for False {}

pub trait BoolOp<B1:Type<Bool>,B2:Type<Bool>> {type F:Type<Bool>;}

pub struct Or;
impl Type<Arrow2<Bool,Bool,Bool>> for Or {}
impl Func2<Bool,Bool,Bool,True,True> for Or {type F=True;}
impl Func2<Bool,Bool,Bool,True,False> for Or {type F=True;}
impl Func2<Bool,Bool,Bool,False,True> for Or {type F=True;}
impl Func2<Bool,Bool,Bool,False,False> for Or {type F=False;}
impl<B1:Type<Bool>,B2:Type<Bool>> BoolOp<B1,B2> for Or where
  Or: Func2<Bool,Bool,Bool,B1,B2>
{type F=<Or as Func2<Bool,Bool,Bool,B1,B2>>::F;}
