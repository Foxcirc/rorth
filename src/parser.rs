
#[allow(unused_assignments)]
pub(crate) mod parser;

pub(crate) mod bytecode;
pub(crate) mod procedure;
pub(crate) mod constant;
pub(crate) mod structure;
pub(crate) mod value;

pub(crate) use {
    parser::*,
    bytecode::*,
    procedure::*,
    constant::*,
    structure::*,
    value::*,
};
