use crate::types::*;
use crate::ast;
use heck::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Param {
    name: String,
    ty: Type,
    optional: bool,
}

#[derive(Clone, Debug)]
enum Type {
    Union(Vec<Type>),
    Simple(String)
}

impl From<ast::Param> for Param {
    fn from(val: crate::ast::Param) -> Self {
        fn parse_ty(s: &str) -> Type {
            Type::Simple(s.replace('.', "::"))
        }
        
        let ty = val.ty;
        let ty = if ty.contains('|') {
            Type::Union(ty.split('|').map(parse_ty).collect())
        } else {
            parse_ty(&ty)
        };
        Self  {
            name: val.name,
            ty,
            optional: val.optional,
        }
    }
}

#[derive(Clone, Debug)]
struct Method {
    name: String,
    ret: String,
    params: Vec<Param>
}

impl From<ast::Method> for Method {
    fn from(val: ast::Method) -> Self {
        
        Self {
            name: escape_rust_keyword(&val.name).to_owned(),
            ret: match val.checked.returnType.into_iter().next().unwrap() {
                ast::Type::Basic(s) => s,
                ast::Type::Array(tys) => "".into(),
            },
            params: val.params.into_iter().map(Param::from).collect(),
        }
    }
}

fn gen_single_fn(func: &Method, w: &mut impl std::io::Write) {
    static VISITED : Mutex<Vec<String>> = Vec::new();
    match func.params.iter().enumerate().find(|(_, p)| p.optional || matches!(p.ty, Type::Union(_))) {
        Some((i, p)) if p.optional => {
            let mut func = func.clone();
            let param = &mut func.params[i];
            param.optional = false;
            gen_single_fn(&func, w);
            func.params.truncate(i);
            gen_single_fn(&func, w);
        },
        Some((i, p)) => {
            let mut func = func.clone();
            if let Type::Union(types) = &mut func.params[i].ty {
                let last = types.pop();
                if let Some(last) = last {
                    gen_single_fn(&func, w);
                    func.params[i].ty = last;
                    gen_single_fn(&func, w);        
                }
            }
        }
        None => {
            let mut args_types = Vec::new();
            for p in &func.params {
                let ty_str = match &p.ty { Type::Simple(s) => s, _ => unreachable!() };
                args_types.push(get_rust_type(ty_str.as_str()));
            }
            let args_tup: String = args_types.iter().map(|t| format!("{},",t)).collect();
            let extern_args:String = args_types.iter().map(|s| format!("_ : {}, ",s.replace("'_",""))).collect();
            let fn_sig = format!("({args_ty}) -> {st_name}",args_ty = args_tup, st_name = func.name.to_camel_case());
            let visited = VISITED.lock().unwrap();
            if visited.contains(&fn_sig) {
                return
            }
            visited.push(fn_sig);
            let return_type = get_rust_ret_ty(&func.ret);
            let js_return_type = if return_type == "()" {
                String::new()
            } else {
                format!("-> {}", return_type)
            };
            write!(w, 
r#"
#[wasm_bindgen]
extern {{
    #[wasm_bindgen(js_name = "{js_org_name}")]
    fn {js_name}({args}) {js_return_type};
}}

#[doc(hidden)]
impl FnOnce<({args_ty})> for {st_name}InternalType {{
    type Output = {return_type};
    extern "rust-call" fn call_once(self, args: ({args_ty})) -> Self::Output {{
        {js_name}.call(args)
    }}
}}

#[doc(hidden)]
impl FnMut<({args_ty})> for {st_name}InternalType {{
    extern "rust-call" fn call_mut(&mut self, args: ({args_ty})) -> Self::Output {{
        {js_name}.call(args)
    }}
}}

#[doc(hidden)]
impl Fn<({args_ty})> for {st_name}InternalType {{
    extern "rust-call" fn call(&self, args: ({args_ty})) -> Self::Output {{
        {js_name}.call(args)
    }}
}}
"#, st_name = func.name.to_camel_case(),js_org_name = func.name , js_name = format!("{}{}", func.name, rand::random::<u32>()),
    args_ty = args_tup, args = extern_args, return_type = return_type,js_return_type=js_return_type
);
        }
    }
}

pub fn gen_global_fn(ast: &ast::Ast, mut w: impl std::io::Write) {
    writeln!(w,"use crate::types::*;");
    writeln!(w,"use crate::constants::*;");
    writeln!(w,"use wasm_bindgen::prelude::*;");
    let mut visited = HashSet::new();
    for (_, file_info) in &ast.classes.files.items {
        for processed in file_info.augmentations
            .as_ref().into_iter()
            .flat_map(|a| &a.items)
            .filter(|(key, _)| *key == "p5")
            .map(|(_, class_info)| &class_info.processed)
        {
            for instance_method in &processed.instanceMethods {
                let name = &instance_method.name;
                if !visited.contains(&instance_method.name.as_str()) {
                    gen_docs(&instance_method.checked.classitem, &mut w);
                    writeln!(w, "
pub static {fn_name}: {st_name}InternalType = {st_name}InternalType;
#[doc(hidden)]
pub struct {st_name}InternalType;
", st_name = escape_rust_keyword(name).to_camel_case(), fn_name = escape_rust_keyword(name));
                    visited.insert(name.as_str());
                }

                gen_single_fn(&instance_method.clone().into(), &mut w);
            }
        }
    }
}



fn gen_docs(d: &ast::Description, w: &mut impl std::io::Write) {
    use scraper::{Html, Selector};
    // write the main description
    if let Some(desc) = &d.description {
        let desc = desc.replace("<input></input>", "input")
            .replace("<button></button>", "button")
            .replace("<image>", "image")
            .replace("<video>", "video")
            .replace("<select></select>", "select");
        writeln!(w, r###"#[doc=r##"{}"##]"###, desc);
    } else {
        writeln!(w, "/// No description available\n");
    }
    if let Some(examples) = &d.example {
        writeln!(w, "///<h2>Examples</h2>\n///\n");
        for ex in examples {
            let frag = Html::parse_fragment(&ex);
            let selector = Selector::parse("code").unwrap();
            for example in frag.select(&selector) {
                writeln!(w, "#[doc=r###\"```rust{}```\"###]", example.text().collect::<String>());
            }
        }
    }
    if let Some(overloads) = &d.overloads {
        let mut params_saved = std::collections::HashMap::new();
        writeln!(w, "/// <h2>Overloads</h2>");
        for ov in overloads {
            for param in &ov.params {
                let desc = Html::parse_fragment(&param.description);
                let desc = params_saved.entry(&param.name).or_insert_with(|| desc.root_element().text().collect::<String>());
                writeln!(w, "///");
                writeln!(w, "#[doc = r##\"<code>{}{}</code> {}\n\"##]", param.name,if param.optional { "?" } else { "" }, desc);
                writeln!(w, "///");
            }
            writeln!(w, "///\n\
                         /// ---\n\
                         ///");
        }
    }

    if let Some(params) = &d.params {
        writeln!(w, "/// <h2>Parameters</h2>");
        for param in params {
            let desc = Html::parse_fragment(&param.description);
            writeln!(w, "///");
            writeln!(w, "#[doc = r##\"<code>{}{}</code> {}\n\"##]", param.name,if param.optional { "?" } else { "" }, desc.root_element().text().collect::<String>());
            writeln!(w, "///");
        }
    }
}


pub fn escape_rust_keyword(s: &str) -> &str {
    match s{
        "match" => "r#match",
        "loop" => "r#loop",
        "box" => "r#box",
        s => s,
    }
}

pub fn gen_enums(ast: &ast::Ast, mut w: impl std::io::Write) {
    for (name, values) in &ast.constants {
        writeln!(w, "#[wasm_bindgen]\npub enum {} {{", name);
        for value in values {
            writeln!(w, "    {} = {},", value[0].to_camel_case(), ast.literals[&value[0]].replace('\'',"\"" ));
        }
        writeln!(w, "}}");
    }
}
