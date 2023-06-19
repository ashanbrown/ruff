use rustpython_parser::ast::{self, Constant, Expr, Ranged};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for direct instantiations of abstract base classes.
///
/// ## Why is this bad?
/// An abstract base class should be used only as the base class for another class and never
/// directly instantiated.
///
/// ## Example
/// ```python
/// import abc
///
/// class Animal(abc.ABC):
///    @abc.abstractmethod
///    def make_sound(self):
///        pass
///
/// sheep = Animal()  # [abstract-class-instantiated]
/// ```
///
/// Use instead:
/// ```python
/// class Animal(abc.ABC):
///     @abc.abstractmethod
///     def make_sound(self):
///         pass
///
/// class Sheep(Animal):
///     def make_sound(self):
///         print("bhaaaaa")
///
/// sheep = Sheep()
///     pass
/// ```
#[violation]
pub struct AbstractClassInstantiated {
    name: String,
}

impl Violation for AbstractClassInstantiated {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name } = self;
        format!("Class `{name}` is abstract and should not be instantiated")
    }
}

/// PLE0110
pub(crate) fn abstract_class_instantiated(checker: &mut Checker, expr: &Expr) {
    match test {
        Expr::Constant(ast::ExprConstant { value, .. }) => match value {
            Constant::Str(value, ..) => {
                checker.diagnostics.push(Diagnostic::new(
                    AssertOnStringLiteral {
                        kind: if value.is_empty() {
                            Kind::Empty
                        } else {
                            Kind::NonEmpty
                        },
                    },
                    test.range(),
                ));
            }
            Constant::Bytes(value) => {
                checker.diagnostics.push(Diagnostic::new(
                    AssertOnStringLiteral {
                        kind: if value.is_empty() {
                            Kind::Empty
                        } else {
                            Kind::NonEmpty
                        },
                    },
                    test.range(),
                ));
            }
            _ => {}
        },
        Expr::JoinedStr(ast::ExprJoinedStr { values, range: _ }) => {
            checker.diagnostics.push(Diagnostic::new(
                AssertOnStringLiteral {
                    kind: if values.iter().all(|value| match value {
                        Expr::Constant(ast::ExprConstant { value, .. }) => match value {
                            Constant::Str(value, ..) => value.is_empty(),
                            Constant::Bytes(value) => value.is_empty(),
                            _ => false,
                        },
                        _ => false,
                    }) {
                        Kind::Empty
                    } else if values.iter().any(|value| match value {
                        Expr::Constant(ast::ExprConstant { value, .. }) => match value {
                            Constant::Str(value, ..) => !value.is_empty(),
                            Constant::Bytes(value) => !value.is_empty(),
                            _ => false,
                        },
                        _ => false,
                    }) {
                        Kind::NonEmpty
                    } else {
                        Kind::Unknown
                    },
                },
                test.range(),
            ));
        }
        _ => {}
    }
}
