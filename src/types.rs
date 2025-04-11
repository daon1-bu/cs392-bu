use std::collections::HashMap;
use crate::utils::*;

/// Type representation for Featherweight Rust
#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Unit,
    Int,
    Ref(Box<Ty>, bool), 
    Box(Box<Ty>),
}


pub type Env = HashMap<Ident, (Ty, Lifetime)>;


pub struct TypeContext {
    pub env: Env,
}

impl TypeContext {
    pub fn new() -> Self {
        TypeContext {
            env: HashMap::new(),
        }
    }

  
    pub fn type_stmt(&mut self, _stmt: &Stmt, _l: Lifetime) {
        todo!()
    }

   
    pub fn type_expr(&self, expr: &Expr, _l: Lifetime) -> Ty {
        match expr {
            Expr::Unit => Ty::Unit,
            Expr::Int(_) => Ty::Int,

            Expr::Lval(lval, _) => {
                let var = &lval.ident;
                let (ty, _) = self.env.get(var)
                    .expect(&format!("Unbound variable: {}", var));
                ty.clone()
            }

            _ => unimplemented!("type_expr: not yet implemented for this expression"),
        }
    }
}
