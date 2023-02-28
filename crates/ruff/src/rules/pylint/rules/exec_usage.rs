use crate::violation::Violation;
use crate::{ast::types::Range, checkers::ast::Checker, registry::Diagnostic};
use ruff_macros::{define_violation, derive_message_formats};
use rustpython_parser::ast::{Expr, ExprKind};

define_violation!(
    /// ## What it does
    /// Checks for use of `exec`.
    ///
    /// ## Why is this bad?
    /// Pylint discourages the use of `exec` it can be dangerous
    /// and slower then executing the code directly.
    ///
    /// ## Example
    /// ```python
    /// exec("print('test')")
    /// ```
    pub struct ExecUsage;
);
impl Violation for ExecUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Don't use exec")
    }
}

pub fn exec_usage(checker: &mut Checker, expr: &Expr) {
    if let ExprKind::Name { id, ctx: _ } = &expr.node {
        if id == "eval" {
            checker
                .diagnostics
                .push(Diagnostic::new(ExecUsage, Range::from_located(expr)));
        }
    }
}
