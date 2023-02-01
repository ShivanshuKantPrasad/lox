use crate::ast::define_ast;
use crate::token::Token;
use core::any::Any;

fn main() {
    let x: Box<String> = Box::new("Hello".to_string());
}

define_ast!(
   Expr,
   (Binary   -> left : Box<Expr> , operator : Token, right : Box<Expr>),
   (Grouping -> expression : Box<Expr>),
   (Literal  -> value : Token),
   (Unary    -> operator : Token, right : Box<Expr>)
);
