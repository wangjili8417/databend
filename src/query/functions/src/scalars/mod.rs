// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_expression::type_check::ALL_SIMPLE_CAST_FUNCTIONS;
use common_expression::types::DataType;
use common_expression::types::NumberDataType;
use common_expression::types::ALL_INTEGER_TYPES;
use common_expression::AutoCastRules;
use common_expression::FunctionRegistry;
use ctor::ctor;

mod arithmetic;
mod arithmetic_modulo;
mod array;
mod boolean;
mod control;
mod datetime;
mod geo;
mod math;
mod tuple;
mod variant;

mod comparison;
mod decimal;
mod hash;
mod other;
mod string;
mod string_multi_args;

pub use comparison::check_pattern_type;
pub use comparison::is_like_pattern_escape;
pub use comparison::PatternType;

use self::comparison::ALL_COMP_FUNC_NAMES;

#[ctor]
pub static BUILTIN_FUNCTIONS: FunctionRegistry = builtin_functions();

fn builtin_functions() -> FunctionRegistry {
    let mut registry = FunctionRegistry::empty();

    register_auto_cast_rules(&mut registry);

    variant::register(&mut registry);
    arithmetic::register(&mut registry);
    array::register(&mut registry);
    boolean::register(&mut registry);
    control::register(&mut registry);
    comparison::register(&mut registry);
    datetime::register(&mut registry);
    math::register(&mut registry);
    string::register(&mut registry);
    string_multi_args::register(&mut registry);
    tuple::register(&mut registry);
    geo::register(&mut registry);
    hash::register(&mut registry);
    other::register(&mut registry);
    decimal::register(&mut registry);

    registry
}

fn register_auto_cast_rules(registry: &mut FunctionRegistry) {
    registry.register_default_cast_rules(GENERAL_CAST_RULES.iter().cloned());
    registry.register_default_cast_rules(CAST_FROM_STRING_RULES.iter().cloned());
    registry.register_default_cast_rules(CAST_FROM_VARIANT_RULES());
    registry.register_auto_try_cast_rules(CAST_FROM_VARIANT_RULES());

    for func_name in ["and", "or", "not", "xor"] {
        for data_type in ALL_INTEGER_TYPES {
            registry.register_additional_cast_rules(func_name, [(
                DataType::Number(*data_type),
                DataType::Boolean,
            )]);
            registry.register_additional_cast_rules(func_name, GENERAL_CAST_RULES.iter().cloned());
            registry
                .register_additional_cast_rules(func_name, CAST_FROM_STRING_RULES.iter().cloned());
            registry.register_additional_cast_rules(func_name, CAST_FROM_VARIANT_RULES());
        }
    }

    for func_name in ALL_SIMPLE_CAST_FUNCTIONS {
        // Disable auto cast from strings or variants.
        registry.register_additional_cast_rules(func_name, GENERAL_CAST_RULES.iter().cloned());
    }

    for func_name in ALL_COMP_FUNC_NAMES {
        // Disable auto cast from strings, e.g., `1 < '1'`.
        registry.register_additional_cast_rules(func_name, GENERAL_CAST_RULES.iter().cloned());
        registry.register_additional_cast_rules(func_name, CAST_FROM_VARIANT_RULES());
    }
}

/// The cast rules for any situation, including comparison functions, joins, etc.
pub const GENERAL_CAST_RULES: AutoCastRules = &[
    (DataType::String, DataType::Timestamp),
    (DataType::String, DataType::Date),
    (DataType::String, DataType::Boolean),
    (DataType::Date, DataType::Timestamp),
    (
        DataType::Number(NumberDataType::UInt8),
        DataType::Number(NumberDataType::UInt16),
    ),
    (
        DataType::Number(NumberDataType::UInt8),
        DataType::Number(NumberDataType::UInt32),
    ),
    (
        DataType::Number(NumberDataType::UInt8),
        DataType::Number(NumberDataType::UInt64),
    ),
    (
        DataType::Number(NumberDataType::UInt8),
        DataType::Number(NumberDataType::Int16),
    ),
    (
        DataType::Number(NumberDataType::UInt8),
        DataType::Number(NumberDataType::Int32),
    ),
    (
        DataType::Number(NumberDataType::UInt8),
        DataType::Number(NumberDataType::Int64),
    ),
    (
        DataType::Number(NumberDataType::UInt8),
        DataType::Number(NumberDataType::Float32),
    ),
    (
        DataType::Number(NumberDataType::UInt8),
        DataType::Number(NumberDataType::Float64),
    ),
    (
        DataType::Number(NumberDataType::UInt16),
        DataType::Number(NumberDataType::UInt32),
    ),
    (
        DataType::Number(NumberDataType::UInt16),
        DataType::Number(NumberDataType::UInt64),
    ),
    (
        DataType::Number(NumberDataType::UInt16),
        DataType::Number(NumberDataType::Int32),
    ),
    (
        DataType::Number(NumberDataType::UInt16),
        DataType::Number(NumberDataType::Int64),
    ),
    (
        DataType::Number(NumberDataType::UInt16),
        DataType::Number(NumberDataType::Float32),
    ),
    (
        DataType::Number(NumberDataType::UInt16),
        DataType::Number(NumberDataType::Float64),
    ),
    (
        DataType::Number(NumberDataType::UInt32),
        DataType::Number(NumberDataType::UInt64),
    ),
    (
        DataType::Number(NumberDataType::UInt32),
        DataType::Number(NumberDataType::Int64),
    ),
    (
        DataType::Number(NumberDataType::UInt32),
        DataType::Number(NumberDataType::Float64),
    ),
    (
        DataType::Number(NumberDataType::UInt64),
        DataType::Number(NumberDataType::Int64),
    ),
    (
        DataType::Number(NumberDataType::UInt64),
        DataType::Number(NumberDataType::Float64),
    ),
    (
        DataType::Number(NumberDataType::Int8),
        DataType::Number(NumberDataType::Int16),
    ),
    (
        DataType::Number(NumberDataType::Int8),
        DataType::Number(NumberDataType::Int32),
    ),
    (
        DataType::Number(NumberDataType::Int8),
        DataType::Number(NumberDataType::Int64),
    ),
    (
        DataType::Number(NumberDataType::Int8),
        DataType::Number(NumberDataType::Float32),
    ),
    (
        DataType::Number(NumberDataType::Int8),
        DataType::Number(NumberDataType::Float64),
    ),
    (
        DataType::Number(NumberDataType::Int16),
        DataType::Number(NumberDataType::Int32),
    ),
    (
        DataType::Number(NumberDataType::Int16),
        DataType::Number(NumberDataType::Int64),
    ),
    (
        DataType::Number(NumberDataType::Int16),
        DataType::Number(NumberDataType::Float32),
    ),
    (
        DataType::Number(NumberDataType::Int16),
        DataType::Number(NumberDataType::Float64),
    ),
    (
        DataType::Number(NumberDataType::Int32),
        DataType::Number(NumberDataType::Int64),
    ),
    (
        DataType::Number(NumberDataType::Int32),
        DataType::Number(NumberDataType::Float64),
    ),
    (
        DataType::Number(NumberDataType::Int64),
        DataType::Number(NumberDataType::Float64),
    ),
    (
        DataType::Number(NumberDataType::Float32),
        DataType::Number(NumberDataType::Float64),
    ),
];

/// The rules for automatic casting from string to other types. For example, they are
/// used to allow `add_hours('2023-01-01 00:00:00', '1')`. But they should be disabled
/// for comparison functions, because `1 < '1'` should be an error.
pub const CAST_FROM_STRING_RULES: AutoCastRules = &[
    (DataType::String, DataType::Number(NumberDataType::UInt8)),
    (DataType::String, DataType::Number(NumberDataType::UInt16)),
    (DataType::String, DataType::Number(NumberDataType::UInt32)),
    (DataType::String, DataType::Number(NumberDataType::UInt64)),
    (DataType::String, DataType::Number(NumberDataType::Int8)),
    (DataType::String, DataType::Number(NumberDataType::Int16)),
    (DataType::String, DataType::Number(NumberDataType::Int32)),
    (DataType::String, DataType::Number(NumberDataType::Int64)),
];

#[allow(non_snake_case)]
pub fn CAST_FROM_VARIANT_RULES() -> impl IntoIterator<Item = (DataType, DataType)> {
    [
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Boolean)),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Date)),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Timestamp)),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::String)),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt8))),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt16))),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt32))),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt64))),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Int8))),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Int16))),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Int32))),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Int64))),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Float32))),
        ),
        (
            DataType::Variant,
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Float64))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Boolean)),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Date)),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Timestamp)),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::String)),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt8))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt16))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt32))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::UInt64))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Int8))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Int16))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Int32))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Int64))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Float32))),
        ),
        (
            DataType::Nullable(Box::new(DataType::Variant)),
            DataType::Nullable(Box::new(DataType::Number(NumberDataType::Float64))),
        ),
    ]
}
