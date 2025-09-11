use std::rc::Rc;

fn main() {
    println!("Hehe")
}

#[derive(Debug, PartialEq, Clone)]
enum Name {
    Global(String),
    Local(i32),
    Quote(i32)
}

#[derive(Debug, PartialEq)]
enum InferableTerm {
    Ann(Box<CheckableTerm>, Type),
    Bound(usize),
    Free(Name),
    App(Box<InferableTerm>, Box<CheckableTerm>)
}

#[derive(Debug, PartialEq)]
enum CheckableTerm {
    Inf(Box<InferableTerm>),
    Lam(Box<CheckableTerm>)
}

#[derive(Debug, PartialEq)]
enum Type {
    TFree(Name),
    Fun(Box<Type>, Box<Type>)
    // add a bound thing here when doing dependent types
}

#[derive(Clone)]
enum Value {
    VLam(Rc<dyn Fn(Value) -> Value>), // we represent lambdas as literally just functions behind the scence
    VNeutral(Neutral)
}

#[derive(Clone)]
enum Neutral {
    NFree(Name),
    NApp(Box<Neutral>, Box<Value>)
}

fn vfree(n : Name) -> Value {
    Value::VNeutral(Neutral::NFree(n))
}

fn vapp(v1: Value, v2: Value) -> Value {
    match v1 {
        Value::VLam(f) => f(v2),
        Value::VNeutral(n) => Value::VNeutral(Neutral::NApp(Box::new(n), Box::new(v2)))
    }
}

type Env = Vec<Value>;

fn evalInferable(term: &InferableTerm, env: &Env) -> Value {
    match term {
        InferableTerm::Ann(e, _) => evalCheckable(&e, env),
        InferableTerm::Free(x) => vfree(x.clone()),
        InferableTerm::Bound(i) => env[*i].clone(),
        InferableTerm::App(e, f) => vapp(evalInferable(e, env), evalCheckable(f, env))
    }
}

fn evalCheckable(term: &CheckableTerm, env: &Env) -> Value {
    match term {
        CheckableTerm::Inf(t) => evalInferable(t, env),
        CheckableTerm::Lam(f) => {
            let anon = |x : Value| -> Value {
                evalCheckable(f, env)
            };
            return Value::VLam(Rc::new(anon))
        }
    }
}