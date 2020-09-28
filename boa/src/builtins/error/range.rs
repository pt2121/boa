//! This module implements the global `RangeError` object.
//!
//! Indicates a value that is not in the set or range of allowable values.
//!
//! More information:
//!  - [MDN documentation][mdn]
//!  - [ECMAScript reference][spec]
//!
//! [spec]: https://tc39.es/ecma262/#sec-native-error-types-used-in-this-standard-rangeerror
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RangeError

use crate::{
    builtins::BuiltIn,
    object::{ConstructorBuilder, ObjectData},
    profiler::BoaProfiler,
    property::Attribute,
    Context, Result, Value,
};

/// JavaScript `RangeError` impleentation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RangeError;

impl BuiltIn for RangeError {
    const NAME: &'static str = "RangeError";

    fn init(context: &mut Context) -> (&'static str, Value, Attribute) {
        let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

        let attribute = Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE;
        let range_error_object = ConstructorBuilder::with_standard_object(
            context,
            Self::constructor,
            context.standard_objects().range_error_object().clone(),
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

impl RangeError {
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
    pub(crate) fn to_string(this: &Value, _: &[Value], ctx: &mut Context) -> Result<Value> {
        let name = this.get_field("name").to_string(ctx)?;
        let message = this.get_field("message").to_string(ctx)?;

        Ok(Value::from(format!("{}: {}", name, message)))
    }
}
