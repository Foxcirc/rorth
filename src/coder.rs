
#[allow(unused_assignments)]
pub(crate) mod coder;

pub(crate) mod bytecode;
pub(crate) mod procedure;
pub(crate) mod constant;
pub(crate) mod structure;
pub(crate) mod value;

pub(crate) use {
    coder::*,
    bytecode::*,
    procedure::*,
    constant::*,
    structure::*,
    value::*,
};
