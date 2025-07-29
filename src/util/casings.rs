// https://github.com/lucacasonato/cases/blob/main/mod.ts

use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[non_exhaustive]
pub enum Casing {
    #[default]
    Snake,
    #[cfg_attr(feature = "serde", serde(alias = "SCREAMING_SNAKE_CASE"))]
    ScreamingSnake,
    #[cfg_attr(feature = "serde", serde(alias = "PascalCase"))]
    Pascal,
    #[cfg_attr(feature = "serde", serde(alias = "camelCase"))]
    Camel,
    #[cfg_attr(feature = "serde", serde(alias = "lowercase", alias = "luacase"))]
    Lower,
    #[cfg_attr(feature = "serde", serde(alias = "UPPERCASE"))]
    Upper,
}

#[macro_export]
macro_rules! match_casings {
    ($casing: expr => $ident: ident) => {{
        use $crate::util::casings::Casing;
        use pastey::paste;
        paste! {
            match $casing {
                Casing::Snake => stringify!([<$ident:snake>]),
                Casing::ScreamingSnake => stringify!([<$ident:snake:upper>]),
                Casing::Pascal => stringify!([<$ident:upper_camel>]),
                Casing::Camel => stringify!([<$ident:lower_camel>]),
                Casing::Lower => stringify!([<$ident:lower>]),
                Casing::Upper => stringify!([<$ident:upper>]),
            }
        }
    }};
}
