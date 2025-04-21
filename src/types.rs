use std::collections::HashMap;
use crate::utils::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Unit,
    Int,
    Box(Box<Type>),
    Ref(Lval, bool),
    Undefined(Box<Type>),
}

#[derive(Clone, Debug)]
pub enum Error {
    Dummy,
    UnboundVar(String),
    InvalidMove,
    InvalidWrite,
}

pub type TypeResult<T> = Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Slot {
    pub tipe: Type,
    pub lifetime: Lifetime,
}

#[derive(Clone, Debug, Default)]
pub struct Env(pub HashMap<Ident, Slot>);

impl Env {
    pub fn insert(&mut self, var: &str, tipe: Type, lifetime: Lifetime) {
        self.0.insert(var.to_string(), Slot { tipe, lifetime });
    }

    pub fn type_lval(&self, lval: &Lval) -> TypeResult<Slot> {
        let Slot { tipe, lifetime } = self.0.get(&lval.ident)
            .ok_or(Error::UnboundVar(lval.ident.clone()))?;
        Ok(Slot {
            tipe: tipe.clone(),
            lifetime: lifetime.clone(), 
        })
    }

    pub fn contained(&self, var: &Ident) -> Option<&Type> {
        let mut current = &self.0.get(var)?.tipe;
        while let Type::Undefined(inner) = current {
            current = inner;
        }
        Some(current)
    }

    pub fn read_prohibited(&self, lval: &Lval) -> bool {
        matches!(self.contained(&lval.ident), Some(Type::Undefined(_)))
    }

    pub fn write_prohibited(&self, lval: &Lval) -> bool {
        match self.contained(&lval.ident) {
            Some(Type::Ref(_, false)) => true,
            Some(Type::Undefined(_)) => true,
            _ => false,
        }
    }

    pub fn moove(&mut self, lval: &Lval) -> TypeResult<()> {
        if self.read_prohibited(lval) {
            return Err(Error::InvalidMove);
        }
        let contained = self.contained(&lval.ident).unwrap().clone();
        self.write(lval, Type::Undefined(Box::new(contained)))
    }

    pub fn muut(&self, lval: &Lval) -> bool {
        matches!(self.contained(&lval.ident), Some(Type::Ref(_, true)))
    }

    pub fn compatible(&self, t1: &Type, t2: &Type) -> bool {
        match (t1, t2) {
            (Type::Int, Type::Int) => true,
            (Type::Unit, Type::Unit) => true,
            (Type::Box(b1), Type::Box(b2)) => self.compatible(b1, b2),
            (Type::Ref(_, m1), Type::Ref(_, m2)) => m1 == m2,
            _ => false,
        }
    }

    pub fn write(&mut self, w: &Lval, tipe: Type) -> TypeResult<()> {
        if self.write_prohibited(w) {
            return Err(Error::InvalidWrite);
        }

        let mut_key = w.ident.clone(); 
        let mut t = &mut self.0.get_mut(&mut_key).ok_or(Error::UnboundVar(mut_key.clone()))?.tipe;

        for _ in 0..w.derefs {
            let inner_lval = match t {
                Type::Ref(inner, _) => inner.ident.clone(), 
                _ => return Err(Error::InvalidWrite),
            };
            t = &mut self.0.get_mut(&inner_lval)
                .ok_or(Error::UnboundVar(inner_lval.clone()))?
                .tipe;
        }

        *t = tipe;
        Ok(())
    }

    pub fn drop(&mut self, l: Lifetime) {
        self.0.retain(|_, slot| slot.lifetime != l);
    }
}

#[derive(Clone, Debug)]
pub struct TypeContext {
    pub env: Env,
}

impl TypeContext {
    pub fn new() -> Self {
        TypeContext {
            env: Env::default(),
        }
    }

    pub fn type_stmt(&mut self, stmt: &Stmt, l: Lifetime) -> TypeResult<()> {
        match stmt {
            Stmt::LetMut(ident, expr) => {
                let t = self.type_expr(&mut expr.clone())?; 
                self.env.insert(ident, t, l);
                Ok(())
            }
            Stmt::Assign(lval, expr) => {
                let t = self.type_expr(&mut expr.clone())?; 
                self.env.write(lval, t)?;
                Ok(())
            }
            Stmt::Expr(expr) => {
                self.type_expr(&mut expr.clone())?;
                Ok(())
            }
        }
    }

    pub fn type_expr(&mut self, expr: &mut Expr) -> TypeResult<Type> {
        match expr {
            Expr::Unit => Ok(Type::Unit),
            Expr::Int(_) => Ok(Type::Int),

            Expr::Lval(lval, copyable) => {
                let slot = self.env.0.get(&lval.ident)
                    .ok_or(Error::UnboundVar(lval.ident.clone()))?;
                match &slot.tipe {
                    Type::Int | Type::Unit => *copyable = true,
                    _ => {}
                }
                Ok(slot.tipe.clone())
            }

            Expr::Box(inner) => {
                let inner_ty = self.type_expr(inner)?;
                Ok(Type::Box(Box::new(inner_ty)))
            }

            _ => unimplemented!("type_expr not yet done"),
        }
    }
}
