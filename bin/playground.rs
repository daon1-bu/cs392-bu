use salt::utils::*;
use salt::eval::Context;
use salt::types::*;

fn main() {
    test_eval_block();
    test_env_lookup();
}

// 🧪 Evaluator test
fn test_eval_block() {
    let mut ctx = Context {
        store: Default::default(),
    };

    let lifetime = Lifetime::global();

    let block_expr = Expr::Block(
        vec![
            Stmt::LetMut("x".to_string(), Expr::Int(1)),
            Stmt::LetMut(
                "y".to_string(),
                Expr::Borrow(
                    Lval {
                        ident: "x".to_string(),
                        derefs: 0,
                    },
                    false,
                ),
            ),
        ],
        Box::new(Expr::Lval(
            Lval {
                ident: "x".to_string(),
                derefs: 0,
            },
            true,
        )),
        lifetime.clone(),
    );

    let result = ctx.eval_expr(&block_expr, lifetime);
    println!("Result from eval: {:?}", result);
}

// 🧪 Type environment test
fn test_env_lookup() {
    let mut env = Env::default();
    env.insert("x", Type::Int, Lifetime(99));

    let slot = env.type_lval(&Lval::new("x", 0)).unwrap();
    println!("Slot from Env: {:?}", slot);
}
