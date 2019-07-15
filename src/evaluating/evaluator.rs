use crate::parsing::Expr;
use std::collections::HashMap;
use crate::format_error;

pub struct Evaluator {
    env: HashMap<String, Expr>
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            env: HashMap::new()
        }
    }
}

impl Evaluator {

   pub fn evaluate(&mut self, expression: Expr) -> Result<Expr, String> {
       match expression {
           Expr::Abstraction(_, _) => Ok(expression),
           Expr::Application(ref left, ref right) => self.reduce_application(&left, &right),
           Expr::Grouping(expr) => self.evaluate(*expr),
           Expr::Variable(_) => Ok(expression),
           Expr::Binding(name, expr) => {
               // Recursively evaluates/binds inner expressions
               let value = self.evaluate(*expr);
               self.env.insert(name, value.clone()?);
               value
           },
           Expr::MetaVariable(token) => match self.env.get(&token.lexeme) {
               Some(expr) => Ok(expr.clone()),
               None => Err(format_error(&format!("Undefined metavariable: {}", token.lexeme), token))
           }
       }
    }

    // Reducible form: (\x.E) N
    // (\x.E) N reduces to
    // E[x->N]
//    pub fn beta_reduce(&self, expression: &Expr) -> Result<Expr, String> {
//        match expression {
//            Expr::Abstraction(_, _) => Ok(expression.clone()),
//            Expr::Application(ref left, ref right) => self.beta_reduce(&self.reduce_application(&left, &right)?),
//            Expr::Grouping(ref expr) => self.beta_reduce(expr),
//            Expr::Variable(_) => Ok(expression.clone()),
//            _ => Err("".to_string())
//        }
//    }


    // pub fn evaluate(&self, expression: Expr) -> Result<Expr, String> {
    //     if let Expr::Application(left, right) = expression {
    //         if let Expr::Abstraction(name, expr) = *left {
    //             self.substitute(&expr, &name, *right)
    //         } else {
    //             Ok(*left)
    //         }
    //     } else {
    //         Ok(expression)
    //     }
    // }

    fn reduce_application(&mut self, left: &Expr, right: &Expr) -> Result<Expr, String> {
        if let Expr::Abstraction(name, expr) = self.evaluate(left.clone())? {
            self.evaluate(self.substitute(&expr, &name, right.clone())?)
        } else {
            Ok(Expr::Application(Box::new(left.clone()), Box::new(right.clone())))
        }
    }

    /* Assuming no naming issues
    E[x->N]
    x[x->N] = N
    (E1 E2)[x->N] = E1[x->N] E2[x->N]
    (\y.E)[x->N] \y.E[x->N]
    */

    pub fn substitute(&self, expression: &Expr, var: &str, with: Expr) -> Result<Expr, String> {
        match expression {
            Expr::Variable(name) => {
                if name == var { Ok(with) }
                else { Ok(expression.clone()) }
            },
            Expr::Application(left, right) =>
                Ok(Expr::Application(
                    Box::new(self.substitute(left, var, with.clone())?),
                    Box::new(self.substitute(right, var, with)?)
                )),
            Expr::Grouping(expr) =>
                self.substitute(expr, var, with),
            Expr::Abstraction(name, expr) =>
                Ok(Expr::Abstraction(name.to_string(), Box::new(self.substitute(expr, var, with)?))),
            _ => unreachable!(),
        }
    }


}