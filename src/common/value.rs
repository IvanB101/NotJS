use core::fmt;

#[derive(PartialEq, Clone)]
pub enum Value {
    None,
    Null,
    Num(f64),
    Str(String),
    Bool(bool),
}

impl Value {
    pub fn extract_num(&self) -> f64 {
        match self {
            Value::Num(num) => *num,
            _ => panic!("It's Nil, you lost the game"),
        }
    }

    pub fn extract_str(&self) -> String {
        match self {
            Value::Str(str) => str.clone(),
            _ => panic!("It's Nil, you lost the game"),
        }
    }

    pub fn extract_bool(&self) -> bool {
        match self {
            Value::Bool(bool) => *bool,
            _ => panic!("It's Nil, you lost the game"),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::None => false,
            Value::Null => false,
            Value::Num(num) => *num != 0.0,
            Value::Str(str) => !str.is_empty(),
            Value::Bool(bool) => *bool,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::None => write!(f, "None"),
            Value::Null => write!(f, "Null"),
            Value::Num(num) => write!(f, "{}", num),
            Value::Str(str) => write!(f, "\"{}\"", str),
            Value::Bool(bool) => write!(f, "{}", bool),
        }
    }
}
