//! Builtins live here, such as Object, String, Math, etc.

pub mod array;
pub mod bigint;
pub mod boolean;
pub mod console;
pub mod date;
pub mod error;
pub mod function;
pub mod global_this;
pub mod infinity;
pub mod json;
pub mod map;
pub mod math;
pub mod nan;
pub mod number;
pub mod object;
pub mod regexp;
pub mod string;
pub mod symbol;
pub mod undefined;

pub(crate) use self::{
    array::Array,
    bigint::BigInt,
    boolean::Boolean,
    console::Console,
    date::Date,
    error::{Error, RangeError, ReferenceError, SyntaxError, TypeError},
    function::BuiltInFunctionObject,
    global_this::GlobalThis,
    infinity::Infinity,
    json::Json,
    map::Map,
    math::Math,
    nan::NaN,
    number::Number,
    object::Object as BuiltInObjectObject,
    regexp::RegExp,
    string::String,
    symbol::Symbol,
    undefined::Undefined,
};
use crate::{
    builtins::function::{Function, FunctionFlags, NativeFunction},
    context::StandardConstructor,
    object::{GcObject, Object, ObjectData, PROTOTYPE},
    property::{Attribute, Property, PropertyKey},
    Context, Value,
};
use std::{
    fmt::{self, Debug},
    string::String as StdString,
};

#[derive(Debug)]
pub struct ObjectBuilder<'context> {
    context: &'context mut Context,
    object: GcObject,
}

impl<'context> ObjectBuilder<'context> {
    pub fn new(context: &'context mut Context) -> Self {
        let object = context.construct_object();
        Self { context, object }
    }

    pub fn static_method(
        &mut self,
        function: NativeFunction,
        name: &str,
        length: usize,
    ) -> &mut Self {
        let mut function = Object::function(
            Function::BuiltIn(function.into(), FunctionFlags::CALLABLE),
            self.context
                .standard_objects()
                .function_object()
                .prototype()
                .into(),
        );
        function.insert_property("length", length.into(), Attribute::all());
        function.insert_property("name", name.into(), Attribute::all());

        self.object
            .borrow_mut()
            .insert_property(name, function.into(), Attribute::all());
        self
    }

    pub fn static_property<K, V>(&mut self, key: K, value: V, attribute: Attribute) -> &mut Self
    where
        K: Into<PropertyKey>,
        V: Into<Value>,
    {
        let property = Property::data_descriptor(value.into(), attribute);
        self.object.borrow_mut().insert(key, property);
        self
    }

    fn build(&mut self) -> Value {
        self.object.clone().into()
    }
}

pub struct ConstructorBuilder<'context> {
    context: &'context mut Context,
    constrcutor_function: NativeFunction,
    constructor_object: GcObject,
    prototype: GcObject,
    name: StdString,
    length: usize,
    callable: bool,
    constructable: bool,
    inherit: Option<Value>,
}

impl Debug for ConstructorBuilder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstructorBuilder")
            .field("name", &self.name)
            .field("length", &self.length)
            .field("constructor", &self.constructor_object)
            .field("prototype", &self.prototype)
            .field("inherit", &self.inherit)
            .field("callable", &self.callable)
            .field("constructable", &self.constructable)
            .finish()
    }
}

impl<'context> ConstructorBuilder<'context> {
    pub fn new(context: &'context mut Context, constructor: NativeFunction) -> Self {
        Self {
            context,
            constrcutor_function: constructor,
            constructor_object: GcObject::new(Object::default()),
            prototype: GcObject::new(Object::default()),
            length: 0,
            name: "[Object]".to_string(),
            callable: true,
            constructable: true,
            inherit: None,
        }
    }

    pub(crate) fn with_standard_object(
        context: &'context mut Context,
        constructor: NativeFunction,
        object: StandardConstructor,
    ) -> Self {
        Self {
            context,
            constrcutor_function: constructor,
            constructor_object: object.constructor,
            prototype: object.prototype,
            length: 0,
            name: "[Object]".to_string(),
            callable: true,
            constructable: true,
            inherit: None,
        }
    }

    pub fn method(&mut self, function: NativeFunction, name: &str, length: usize) -> &mut Self {
        let mut function = Object::function(
            Function::BuiltIn(function.into(), FunctionFlags::CALLABLE),
            self.context
                .standard_objects()
                .function_object()
                .prototype()
                .into(),
        );
        function.insert_property("length", length.into(), Attribute::all());
        function.insert_property("name", name.into(), Attribute::all());

        self.prototype
            .borrow_mut()
            .insert_property(name, function.into(), Attribute::all());
        self
    }

    pub fn static_method(
        &mut self,
        function: NativeFunction,
        name: &str,
        length: usize,
    ) -> &mut Self {
        let mut function = Object::function(
            Function::BuiltIn(function.into(), FunctionFlags::CALLABLE),
            self.context
                .standard_objects()
                .function_object()
                .prototype()
                .into(),
        );
        function.insert_property("length", length.into(), Attribute::all());
        function.insert_property("name", name.into(), Attribute::all());

        self.constructor_object.borrow_mut().insert_property(
            name,
            function.into(),
            Attribute::all(),
        );
        self
    }

    pub fn property<K, V>(&mut self, key: K, value: V, attribute: Attribute) -> &mut Self
    where
        K: Into<PropertyKey>,
        V: Into<Value>,
    {
        let property = Property::data_descriptor(value.into(), attribute);
        self.prototype.borrow_mut().insert(key, property);
        self
    }

    pub fn static_property<K, V>(&mut self, key: K, value: V, attribute: Attribute) -> &mut Self
    where
        K: Into<PropertyKey>,
        V: Into<Value>,
    {
        let property = Property::data_descriptor(value.into(), attribute);
        self.constructor_object.borrow_mut().insert(key, property);
        self
    }

    pub fn length(&mut self, length: usize) -> &mut Self {
        self.length = length;
        self
    }

    pub fn name<N>(&mut self, name: N) -> &mut Self
    where
        N: Into<StdString>,
    {
        self.name = name.into();
        self
    }

    pub fn callable(&mut self, callable: bool) -> &mut Self {
        self.callable = callable;
        self
    }

    pub fn constructable(&mut self, constructable: bool) -> &mut Self {
        self.constructable = constructable;
        self
    }

    pub fn inherit(&mut self, prototype: Value) -> &mut Self {
        assert!(prototype.is_object() || prototype.is_null());
        self.inherit = Some(prototype);
        self
    }

    pub fn context(&mut self) -> &'_ mut Context {
        self.context
    }

    pub fn build(&mut self) -> GcObject {
        // Create the native function
        let function = Function::BuiltIn(
            self.constrcutor_function.into(),
            FunctionFlags::from_parameters(self.callable, self.constructable),
        );

        let length = Property::data_descriptor(
            self.length.into(),
            Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT,
        );
        let mut name = StdString::new();
        std::mem::swap(&mut self.name, &mut name);
        let name = Property::data_descriptor(
            name.into(),
            Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::PERMANENT,
        );

        {
            let mut constructor = self.constructor_object.borrow_mut();
            constructor.data = ObjectData::Function(function);
            constructor.insert("length", length);
            constructor.insert("name", name);

            constructor.set_prototype_instance(
                self.context
                    .standard_objects()
                    .function_object()
                    .prototype()
                    .into(),
            );

            constructor.insert_property(PROTOTYPE, self.prototype.clone().into(), Attribute::all());
        }

        {
            let mut prototype = self.prototype.borrow_mut();
            prototype.insert_property(
                "constructor",
                self.constructor_object.clone().into(),
                Attribute::all(),
            );

            if let Some(proto) = self.inherit.take() {
                prototype.set_prototype_instance(proto);
            } else {
                prototype.set_prototype_instance(
                    self.context
                        .standard_objects()
                        .object_object()
                        .prototype()
                        .into(),
                );
            }
        }

        self.constructor_object.clone()
    }
}

pub trait BuiltIn {
    /// The binding name of the property.
    const NAME: &'static str;

    fn attribute() -> Attribute {
        Attribute::all()
    }
    fn init(context: &mut Context) -> (&'static str, Value, Attribute);
}

/// Initializes builtin objects and functions
#[inline]
pub fn init(context: &mut Context) {
    let globals2 = [
        // Global properties.
        Undefined::init,
        Infinity::init,
        NaN::init,
        GlobalThis::init,
        BuiltInFunctionObject::init,
        BuiltInObjectObject::init,
        Math::init,
        Json::init,
        Console::init,
        Array::init,
        BigInt::init,
        Boolean::init,
        Date::init,
        Map::init,
        Number::init,
        String::init,
        RegExp::init,
        Symbol::init,
        Error::init,
        RangeError::init,
        ReferenceError::init,
        TypeError::init,
        SyntaxError::init,
    ];

    let global_object = if let Value::Object(global) = context.global_object() {
        global.clone()
    } else {
        unreachable!("global object should always be an object")
    };

    for init in &globals2 {
        let (name, value, attribute) = init(context);
        let property = Property::data_descriptor(value, attribute);
        global_object.borrow_mut().insert(name, property);
    }
}
