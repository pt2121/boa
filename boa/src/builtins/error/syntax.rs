//! This module implements the global `SyntaxError` object.
//!
//! The SyntaxError object represents an error when trying to interpret syntactically invalid code.
//! It is thrown when the JavaScript engine encounters tokens or token order that does not conform
//! to the syntax of the language when parsing code.
//!
//! More information:
//!  - [MDN documentation][mdn]
//!  - [ECMAScript reference][spec]
//!
//! [spec]: https://tc39.es/ecma262/#sec-native-error-types-used-in-this-standard-syntaxerror
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SyntaxError

use crate::{
    builtins::BuiltIn,
    object::{ConstructorBuilder, ObjectData},
    profiler::BoaProfiler,
    property::Attribute,
    Context, Result, Value,
};

/// JavaScript `SyntaxError` impleentation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SyntaxError;

impl BuiltIn for SyntaxError {
    const NAME: &'static str = "SyntaxError";

    fn attribute() -> Attribute {
        Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE
    }

    fn init(context: &mut Context) -> (&'static str, Value, Attribute) {
        let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

        let attribute = Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE;
        let range_error_object = ConstructorBuilder::with_standard_object(
            context,
            Self::constructor,
            context.standard_objects().syntax_error_object().clone(),
        )
        .name(Self::NAME)
        .length(Self::LENGTH)
        .property("name", Self::NAME, attribute)
        .property("message", "", attribute)
        .method(Self::to_string, "toString", 0)
        .build();

        (Self::NAME, range_error_object.into(), Self::attribute())
    }
}

impl SyntaxError {
    /// The amount of arguments this function object takes.
    pub(crate) const LENGTH: usize = 1;

    /// Create a new error object.
    pub(crate) fn constructor(this: &Value, args: &[Value], ctx: &mut Context) -> Result<Value> {
        if let Some(message) = args.get(0) {
            this.set_field("message", message.to_string(ctx)?);
        }

        // This value is used by console.log and other routines to match Object type
        // to its Javascript Identifier (global constructor method name)
        this.set_data(ObjectData::Error);
        Err(this.clone())
    }

    /// `Error.prototype.toString()`
    ///
    /// The toString() method returns a string representing the specified Error object.
    ///
    /// More information:
    ///  - [MDN documentation][mdn]
    ///  - [ECMAScript reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-error.prototype.tostring
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/toString
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_string(this: &Value, _: &[Value], _: &mut Context) -> Result<Value> {
        let name = this.get_field("name");
        let message = this.get_field("message");
        // FIXME: This should not use `.display()`
        Ok(format!("{}: {}", name.display(), message.display()).into())
    }
}
