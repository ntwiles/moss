use super::{control_op::ControlOp, resolved_value::ResolvedValue, scope_stack::ScopeStack};

pub struct Context {
    pub control_stack: Vec<ControlOp>,
    pub value_stack: Vec<ResolvedValue>,
    pub scope_stack: ScopeStack,
}
