use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::Stmt;
use ruff_python_ast::identifier::Identifier;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for functions in stub (`.pyi`) files that contain multiple
/// statements.
///
/// ## Why is this bad?
/// Stub files are never executed, and are only intended to define type hints.
/// As such, functions in stub files should not contain functional code, and
/// should instead contain only a single statement (e.g., `...`).
///
/// ## Example
///
/// ```pyi
/// def function():
///     x = 1
///     y = 2
///     return x + y
/// ```
///
/// Use instead:
///
/// ```pyi
/// def function(): ...
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct StubBodyMultipleStatements;

impl Violation for StubBodyMultipleStatements {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Function body must contain exactly one statement".to_string()
    }
}

/// PYI048
pub(crate) fn stub_body_multiple_statements(checker: &Checker, stmt: &Stmt, body: &[Stmt]) {
    if body.len() > 1 {
        checker.report_diagnostic(Diagnostic::new(
            StubBodyMultipleStatements,
            stmt.identifier(),
        ));
    }
}
