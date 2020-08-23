macro_rules! function {
    (
        $vis:vis 
        fn $fn_name:ident
        js $js_name:ident
        struct $st_name:ident
        { $($req_arg:ident : $req_ty:ty),* }
        { $($opt_arg:ident : $opt_ty:ty = $opt_val:expr),* }
    ) => {
        $vis struct $st_name {
            $($req_arg : $req_ty,)*
            $($opt_arg : $opt_ty,)*
        }

        impl $st_name {
            $(
                $vis fn $opt_arg(&mut self, $opt_arg: $opt_ty) -> &mut Self {
                    self . $opt_arg = $opt_arg;
                    self
                }
            )*
        }

        impl Drop for $st_name {
            fn drop(&mut self) {
                #[wasm_bindgen::prelude::wasm_bindgen]
                extern "C" {
                    fn $js_name($($req_arg: $req_ty,)*$($opt_arg:$opt_ty,)*);
                }
                $js_name($(self . $req_arg,)*$(self . $opt_arg,)*)
            }
        }
        $vis fn $fn_name($($req_arg : $req_ty),*) -> $st_name {
            $st_name {
                $($req_arg,)*
                $($opt_arg:$opt_val,)*
            }
        }
    };
}

