use std::{cell::RefCell, rc::Rc};

use crate::translate::{TypeInference, TypeInstance};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinType {
    I32,
    I64,
    Bool,
    Char,
    Never,
}

impl BuiltinType {
    fn inference(self) -> TypeInference {
        TypeInference::Simple(match self {
            Self::I32 => TypeInstance::I32,
            Self::I64 => TypeInstance::I64,
            Self::Bool => TypeInstance::Bool,
            Self::Char => TypeInstance::Char,
            Self::Never => TypeInstance::Never,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Builtin {
    pub name: &'static str,
    pub args: &'static [BuiltinType],
    pub result: BuiltinType,
    pub c_definition: &'static str,
}

impl Builtin {
    pub fn type_inference(&self) -> TypeInference {
        TypeInference::Function {
            args: self
                .args
                .iter()
                .map(|ty| Rc::new(RefCell::new(ty.inference())))
                .collect(),
            result: Rc::new(RefCell::new(self.result.inference())),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/builtin_modules.rs"));
