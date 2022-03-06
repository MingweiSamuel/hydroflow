use anyhow::bail;

#[derive(Debug, Clone)]
struct Parser {
}

#[derive(Debug, Clone)]
pub enum Sexp {
    Atom(String),
    List(Vec<Sexp>, char),
    String(String),
}

impl Sexp {
    pub fn expect_atom(self) -> Result<String, anyhow::Error> {
        match self {
            Sexp::Atom(r) => Ok(r),
            _ => bail!("expected atom"),
        }
    }

    pub fn expect_list(self) -> Result<(Vec<Sexp>, char), anyhow::Error> {
        match self {
            Sexp::List(v, ch) => Ok((v, ch)),
            _ => bail!("expected list"),
        }
    }

    pub fn expect_string(self) -> Result<String, anyhow::Error> {
        match self {
            Sexp::String(r) => Ok(r),
            _ => bail!("expected string"),
        }
    }
}

pub trait FromSexp: Sized {
    fn from_sexp(s: Sexp) -> Result<Self, anyhow::Error>;
}

impl FromSexp for Sexp {
    fn from_sexp(s: Sexp) -> Result<Self, anyhow::Error> {
        Ok(s)
    }
}

impl ToSexp for Sexp {
    fn to_sexp(&self) -> Sexp {
        self.clone()
    }
}

pub trait ToSexp {
    fn to_sexp(&self) -> Sexp;
}

impl<I: FromSexp> FromSexp for Vec<I> {
    fn from_sexp(s: Sexp) -> Result<Self, anyhow::Error> {
        let (v, _) = s.expect_list()?;
        v.into_iter()
            .map(|s| I::from_sexp(s))
            .collect::<Result<Self, _>>()
    }
}

impl<I: ToSexp> ToSexp for Vec<I> {
    fn to_sexp(&self) -> Sexp {
        Sexp::List(self.iter().map(|i| i.to_sexp()).collect(), '[')
    }
}

impl<A: FromSexp, B: FromSexp> FromSexp for (A, B) {
    fn from_sexp(s: Sexp) -> Result<Self, anyhow::Error> {
        let (v, _) = s.expect_list()?;
        if v.len() != 2 {
            bail!("expected pair");
        }
        // TODO: remove clone
        Ok((A::from_sexp(v[0].clone())?, B::from_sexp(v[1].clone())?))
    }
}

impl<A: ToSexp, B: ToSexp> ToSexp for (A, B) {
    fn to_sexp(&self) -> Sexp {
        Sexp::List(vec![self.0.to_sexp(), self.1.to_sexp()], '(')
    }
}

macro_rules! impl_from_fromstr {
    ($name:ident) => {
        impl FromSexp for $name {
            fn from_sexp(s: Sexp) -> Result<Self, anyhow::Error> {
                Ok(s.expect_atom()?.parse::<$name>()?)
            }
        }

        impl ToSexp for $name {
            fn to_sexp(&self) -> Sexp {
                Sexp::Atom(format!("{}", self))
            }
        }
    };
}

impl_from_fromstr!(i64);
impl_from_fromstr!(usize);
impl_from_fromstr!(String);

#[derive(Debug, Clone)]
struct LitString(String);

impl FromSexp for LitString {
    fn from_sexp(s: Sexp) -> Result<Self, anyhow::Error> {
        Ok(LitString(s.expect_string()?.parse::<String>()?))
    }
}

impl ToSexp for LitString {
    fn to_sexp(&self) -> Sexp {
        Sexp::String(self.0.to_string())
    }
}

impl ToSexp for () {
    fn to_sexp(&self) -> Sexp {
        Sexp::List(Vec::new(), '(')
    }
}
