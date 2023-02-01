macro_rules! define_ast {
    () => {};
    ($class:ident, $( ($rule:ident -> $($name:ident : $type:ty),+ ) ),+) => {
        use paste::paste;
        use crate::error::LoxError;
        pub enum $class {
            $($rule(paste!([< $rule $class >]))),+
        }
        impl Expr {
            pub fn accept<T>(&self, paste!([<$class:lower _visitor>]): &dyn ExprVisitor<T>)
                             -> Result<T, LoxError>{
                match self {
                    $($class::$rule(v) => v.accept(paste!([<$class:lower _visitor>])),)+
                }
            }
        }
        paste!(pub trait [< $class Visitor >]<T> {
                $(fn [< visit_ $rule:lower _ $class:lower> ](&self, expr: &[< $rule Expr >])
                                                             -> Result<T, LoxError>;)+
        });
        paste!($(pub struct [<$rule Expr>] {
            $(pub $name: $type,)+
        })+);
        paste!($(
            impl [< $rule Expr >]{
                pub fn accept<T>(&self, visitor: &dyn[< $class Visitor>]<T>)
                                 -> Result<T, LoxError> {
                    visitor.[< visit_ $rule:lower _ $class:lower> ](self)
                }
            }
        )+);
    }
}

pub(crate) use define_ast;
