
#[allow(unused_assignments)]
pub(crate) mod parser;
pub(crate) mod bytecode;
pub(crate) mod proc;
pub(crate) mod r#const;
pub(crate) mod r#struct;
pub(crate) mod value;

pub(crate) use {
    parser::*,
    bytecode::*,
    proc::*,
    r#const::*,
    r#struct::*,
    value::*,
};
