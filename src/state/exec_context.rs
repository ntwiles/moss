use crate::{interpreter::resolved_value::ResolvedValue, scopes::scope_stack::ScopeStack};

use super::control_op::ControlOp;

pub struct ExecContext {
    pub control_stack: Vec<ControlOp>,
    pub value_stack: Vec<ResolvedValue>,
    pub scope_stack: ScopeStack<ResolvedValue>,
}
