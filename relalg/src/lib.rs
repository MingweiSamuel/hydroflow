mod runtime;
mod sexp;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Datum {
    Int(i64),
    String(String),
    Bool(bool),
}

impl Datum {
    fn unwrap_int(self) -> i64 {
        if let Datum::Int(i) = self {
            i
        } else {
            panic!("was not int")
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ScalarExpr {
    Literal(Datum),
    ColRef(usize),
    Eq(Box<ScalarExpr>, Box<ScalarExpr>),
    Plus(Box<ScalarExpr>, Box<ScalarExpr>),
}

impl ScalarExpr {
    pub fn eval(&self, data: &[Datum]) -> Datum {
        match self {
            ScalarExpr::Literal(d) => d.clone(),
            ScalarExpr::ColRef(u) => data[*u].clone(),
            ScalarExpr::Eq(a, b) => {
                let a = a.eval(data);
                let b = b.eval(data);
                Datum::Bool(a == b)
            }
            ScalarExpr::Plus(a, b) => {
                let a = a.eval(data).unwrap_int();
                let b = b.eval(data).unwrap_int();
                Datum::Int(a + b)
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum RelExpr {
    Values(Vec<Vec<ScalarExpr>>),
    Filter(Vec<ScalarExpr>, Box<RelExpr>),
    Project(Vec<ScalarExpr>, Box<RelExpr>),
}
