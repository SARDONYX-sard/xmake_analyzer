//! Types for xmake scripts, in the notation of LuaCATS as LuaLS (sumneko)
//! writes them: the [`Type`]s themselves, and their inference from the
//! stubs, the literals and the bindings.

mod expected;
mod infer;
mod scripts;
mod ty;

use xmake_project::{AnalyzedFile, CodeContext, Project, annotations, context, resolve, tables};

pub use crate::{
    expected::{Expected, compared_with, expected_params, nth_expected},
    infer::{
        annotated, call_params, called_declaration, called_stub, expression_before,
        function_returns, global_function, global_function_name, initializer, is_reassigned,
        nth_value_type, type_of, variable_type,
    },
    scripts::{ScriptArg, script_args, script_overloads, script_param},
    ty::{Type, luacats_param, luacats_prototype},
};
