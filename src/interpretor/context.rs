use crate::scope_stack::ScopeStack;

use super::{control_op::ControlOp, resolved_value::ResolvedValue};

pub struct Context {
    pub control_stack: Vec<ControlOp>,
    pub value_stack: Vec<ResolvedValue>,
    pub scope_stack: ScopeStack,
}
