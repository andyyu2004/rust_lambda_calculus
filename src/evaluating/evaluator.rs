use std::collections::{HashMap, HashSet};

use crate::format_error;
use crate::parsing::Expr;
use crate::lexing::Token;

pub struct Evaluator {
    env: HashMap<String, Expr>,
    names: HashSet<String>
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            env: HashMap::new(),
            names: HashSet::new()
        }
    }
}

impl Evaluator {

    pub fn evaluate(&mut self, expression: Expr) -> Result<Expr, String> {
        self.names.clear();
        let expr = self.expand_bindings(&expression)?;
        println!("Expanded (parenthesized): {:?}", expr);
        println!("Expanded: {}", expr);
        println!("names: {:?}", self.names);
        self.beta_reduce(expr)
    }

    fn expand_bindings(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::Variable(name) => {
                self.names.insert(name.clone());
                Ok(expr.clone())
            },
            Expr::Abstraction(name, expr) => {
                self.names.insert(name.clone());
                Ok(Expr::Abstraction(name.to_string(), Box::new(self.expand_bindings(expr)?)))
            },
            Expr::Application(left, right) => Ok(Expr::Application(
                Box::new(self.expand_bindings(left)?),
                Box::new(self.expand_bindings(right)?),
            )),
            Expr::Grouping(expr) => Ok(Expr::Grouping(Box::new(self.expand_bindings(expr)?))),
            Expr::Binding(x, expr) => Ok(Expr::Binding(x.clone(), Box::new(self.expand_bindings(expr)?))),
            Expr::MetaVariable(token) => Ok(Expr::Grouping(
                Box::new(self.evaluate_meta_variable(&token)?)
            ))
        }
    }

    fn beta_reduce(&mut self, expression: Expr) -> Result<Expr, String> {
        match expression {
            Expr::Abstraction(_, _) => Ok(expression),
            Expr::Application(ref left, ref right) => self.reduce_application(&left, &right),
            Expr::Grouping(expr) => self.beta_reduce(*expr),
            Expr::Variable(_) => Ok(expression),
            Expr::Binding(name, expr) => {
                // Recursively evaluates/binds inner expressions
                let value = self.beta_reduce(*expr);
                self.env.insert(name, value.clone()?);
                value
            },
            Expr::MetaVariable(_) => unreachable!("Beta-reducing metavariable")

        }
    }

    fn evaluate_meta_variable(&self, token: &Token) -> Result<Expr, String> {
        match self.env.get(&token.lexeme) {
            Some(expr) => Ok(expr.clone()),
            None => Err(format_error(&format!("Undefined metavariable: {}", token.lexeme), token))
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
        // if expr is redex
        if let Expr::Abstraction(name, expr) = self.beta_reduce(left.clone())? {
            self.beta_reduce(self.substitute(&expr, &name, right)?)
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

    // Doesn't consider bound variables etc...
//    pub fn substitute(&self, expression: &Expr, var: &str, with: &Expr) -> Result<Expr, String> {
//        match expression {
//            Expr::Variable(name) => {
//                if name == var { Ok(with.clone()) } else { Ok(expression.clone()) }
//            }
//            Expr::Application(left, right) =>
//                Ok(Expr::Application(
//                    Box::new(self.substitute(left, var, with)?),
//                    Box::new(self.substitute(right, var, with)?),
//                )),
//            Expr::Grouping(expr) =>
//                self.substitute(expr, var, with),
//            Expr::Abstraction(name, expr) =>
//                Ok(Expr::Abstraction(name.to_string(), Box::new(self.substitute(expr, var, with)?))),
//            _ => unreachable!(),
//        }
//    }

    pub fn substitute(&self, expression: &Expr, var: &str, with: &Expr) -> Result<Expr, String> {
        match expression {
            Expr::Variable(name) => {
                if name == var { Ok(with.clone()) } else { Ok(expression.clone()) }
            }
            Expr::Application(left, right) =>
                Ok(Expr::Application(
                    Box::new(self.substitute(left, var, with)?),
                    Box::new(self.substitute(right, var, with)?),
                )),
            Expr::Grouping(expr) =>
                self.substitute(expr, var, with),
            Expr::Abstraction(name, expr) => {
                if name == var { // Don't substitute bound variables
                    Ok(expression.clone())
                } else if !Evaluator::is_free(expr, name) {
                    Ok(Expr::Abstraction(name.to_string(), Box::new(self.substitute(expr, var, with)?)))
                } else {
                    // name is free in expr thus require rename
                    let new_name = self.generate_name();
                    let new_expr = Evaluator::alpha_rename(expr, name, &new_name);
                    Ok(Expr::Abstraction(new_name, Box::new(self.substitute(&new_expr, var, with)?)))
                }
            },
            _ => unreachable!(),
        }
    }

    fn generate_name(&self) -> String {
        println!("Rename");
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            let str = char::to_string(&c);
            if self.names.contains(&str) { continue; }
            return str
        }
        panic!("Ran out of variable names")
    }

    // Alpha conversion
    pub fn alpha_rename(expression: &Expr, from: &String, to: &String) -> Expr {
        match expression {
            Expr::Variable(name) => if name == from { Expr::Variable(to.clone()) } else { expression.clone() },
            Expr::Application(left, right) => Expr::Application(
                Box::new(Evaluator::alpha_rename(left, from,to)),
                Box::new(Evaluator::alpha_rename(right, from, to)),
            ),
            Expr::Grouping(expr) => Expr::Grouping(Box::new(Evaluator::alpha_rename(expr, from, to))),
            Expr::Abstraction(name, expr) => if name == from {
                Expr::Abstraction(to.clone(), Box::new(Evaluator::alpha_rename(expr, from, to)))
            } else {
                Expr::Abstraction(name.clone(), Box::new(Evaluator::alpha_rename(expr, from, to)))
            },
            Expr::Binding(name, expr) => Expr::Binding(name.clone(), Box::new(Evaluator::alpha_rename(expr, from, to))),
            _ => unreachable!("Alpha renaming metavariable or binding")

        }
    }

    /* x is free in E if
    E = x
    E = \y.E' and x is free in E'
    */
    pub fn is_free(expression: &Expr, var: &str) -> bool {
        match expression {
            Expr::Variable(_) => true,
            Expr::Abstraction(name, expr) => name != var && Evaluator::is_free(expr, var),
            Expr::Application(left, right) => Evaluator::is_free(left, var) || Evaluator::is_free(right, var),
            Expr::Grouping(expr) => Evaluator::is_free(expr, var),
            _ => unreachable!("Checking free variable in binding or metavariable")
        }
    }

}