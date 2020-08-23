pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const TRANSPARENT: Self = Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
}
macro_rules! from {
    ($ident:pat = $ty:ty => $self_:ty $block:block) => {
        impl From<$ty> for $self_ {
            fn from($ident: $ty) -> Self {
                $block
            }
        }
    };
}

from!((red, green, blue) = (u8, u8, u8) => Color {
    Color { red, green ,blue , alpha: 255 }
});

from!((red, green, blue, alpha) = (u8, u8, u8, u8) => Color {
    Color { red, green ,blue , alpha }
});

from!(grey = u8 => Color {
    Color { red: grey, green: grey, blue: grey, alpha: grey }
});

from!((grey, alpha) = (u8, u8) => Color {
    Color { red: grey, green: grey, blue: grey, alpha }
});

pub fn background(color: impl Into<Color>) {
    let c = color.into();
    bindings::background(c.red, c.green, c.blue, c.alpha);
}

mod bindings {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        pub fn background(r: u8, g: u8, b: u8, a: u8);
    }
}
