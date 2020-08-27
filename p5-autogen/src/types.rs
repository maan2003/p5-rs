use std::borrow::Cow;

pub fn get_rust_type(ty: &str) -> Cow<'_, str> {
    match ty {
        "number" | "Number" => "f64".into(),
        "String" | "string" => "&'_ str".into(),
        "Integer" => "i32".into(),
        "boolean" |"Boolean" => "bool".into(),
        "Object" => "JsValue".into(),
        "p5" => "()".into(),
        "Any" |"any" => "JsValue".into(),
        "void" => "()".into(),
        "any[]" | "Array" => "js_sys::Array".into(),
        "object" => "js_sys::Object".into(),
        "Number[]" => "&'_ [f64]".into(),
        "String[]" => "js_sys::Array".into(),
        "Function" => "js_sys::Function".into(),
        "Promise<any>" => "js_sys::Promise".into(),
        "" => "()".into(),
        other if other.starts_with("function") || other.starts_with("Function") => {
            "js_sys::Function".into()            
        }
        other if other.starts_with("p5.") => {
            format!("&'_ {}", &other[3..]).into()
        }
        other if other.starts_with("p5::") => {
            format!("&'_ {}", &other[4..]).into()
        },
        other => {
            println!("{}", other);
            other.into()
        },
    }
}

pub fn get_rust_ret_ty(s: &str) -> Cow<'_, str> {
    match s {
        "String" | "string" => "String".into(),
        "Number[]" => "Vec<f64>".into(),
        "String[]" => "Vec<String>".into(),
        other => get_rust_type(s).replace("&'_", "").into(),
    }
}
