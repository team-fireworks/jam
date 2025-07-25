// https://github.com/lucacasonato/cases/blob/main/mod.ts

// use fancy_regex::Regex;
// use itertools::intersperse;
// use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Casing {
    #[default]
    Snake,
    #[serde(alias = "SCREAMING_SNAKE_casing")]
    ScreamingSnake,
    // #[serde(alias = "kebab-case")]
    // Kebab,
    // #[serde(alias = "SCREAMING-KEBAB-CASE")]
    // ScreamingKebab,
    #[serde(alias = "PascalCase")]
    Pascal,
    #[serde(alias = "camelCase")]
    Camel,
    #[serde(alias = "lowercase", alias = "luacase")]
    Lower,
    #[serde(alias = "UPPERCASE")]
    Upper,
}

#[macro_export]
macro_rules! match_casings {
    ($casing: expr => $ident: ident) => {{
        use crate::util::casings::Casing;
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

// lazy_static! {
//     static ref SPLIT_REGEX: Regex =
//         Regex::new(r#"[^\p{Lu}_\-\s]+|\p{Lu}+(?![^\p{Lu}_\-\s])|\p{Lu}[^\p{Lu}_\-\s]*"#).unwrap();
// }

// TODO: dont use regex bruh
// pub fn split(haystack: &str) -> Vec<&str> {
//     SPLIT_REGEX
//         .captures_iter(haystack)
//         .map(|c| c.unwrap().get(0).unwrap().as_str())
//         .collect()
// }

// pub trait CasingExt {
//     fn to_snake_casing(&self) -> String;
//     fn to_screaming_snake_casing(&self) -> String;
//     fn to_kebab_casing(&self) -> String;
//     fn to_screaming_kebab_casing(&self) -> String;
//     fn to_pascal_casing(&self) -> String;
//     fn to_camel_casing(&self) -> String;
//     fn to_lower_casing(&self) -> String;
//     fn to_upper_casing(&self) -> String;
//     fn to_casing(&self, casing: &Casing) -> String;
// }

// FIXME: intersperse method in std soon...
// impl CasingExt for Vec<String> {
//     fn to_snake_casing(&self) -> String {
//         intersperse(self.iter().map(|s| s.to_uppercase()), "_".to_string()).collect()
//     }

//     fn to_screaming_snake_casing(&self) -> String {
//         intersperse(self.iter().map(|s| s.to_uppercase()), "_".to_string()).collect()
//     }

//     fn to_kebab_casing(&self) -> String {
//         intersperse(self.iter().map(|s| s.to_lowercase()), "-".to_string()).collect()
//     }

//     fn to_screaming_kebab_casing(&self) -> String {
//         intersperse(self.iter().map(|s| s.to_uppercase()), "-".to_string()).collect()
//     }

//     fn to_pascal_casing(&self) -> String {
//         self.iter()
//             .map(|s| {
//                 if s == s.to_uppercase() {
//                     s
//                 } else {
//                     format!(
//                         "{}{}",
//                         s.grapheme.unwrap(),
//                         s.get(1..).unwrap().to_lowercase()
//                     )
//                 }
//             })
//             .collect()
//     }

//     fn to_camel_casing(&self) -> String {
//         todo!()
//     }

//     fn to_lower_casing(&self) -> String {
//         self.iter().map(|s| s.to_lowercase()).collect()
//     }

//     fn to_upper_casing(&self) -> String {
//         self.iter().map(|s| s.to_uppercase()).collect()
//     }

//     fn to_casing(&self, casing: &Casing) -> String {
//         match casing {
//             Casing::Snake => self.to_snake_casing(),
//             Casing::ScreamingSnake => self.to_screaming_snake_casing(),
//             Casing::Kebab => self.to_kebab_casing(),
//             Casing::ScreamingKebab => self.to_screaming_kebab_casing(),
//             Casing::Pascal => self.to_pascal_casing(),
//             Casing::Camel => self.to_camel_casing(),
//             Casing::Lower => self.to_lower_casing(),
//             Casing::Upper => self.to_upper_casing(),
//         }
//     }
// }

// mod tests {
//     #[test]
//     fn split() {
//         assert_eq!(super::split("a b c"), vec!["a", "b", "c"]);
//         assert_eq!(super::split("alphaBetaGamma"), vec![
//             "alpha", "Beta", "Gamma"
//         ]);
//         assert_eq!(super::split("alpha_beta_gamma"), vec![
//             "alpha", "beta", "gamma"
//         ]);
//         assert_eq!(super::split("alpha-beta-gamma"), vec![
//             "alpha", "beta", "gamma"
//         ]);
//         assert_eq!(super::split("parseURL"), vec!["parse", "URL"]);
//         assert_eq!(super::split("parseURLFunction"), vec![
//             "parse", "URL", "Function"
//         ]);
//         assert_eq!(super::split("parse__beta"), vec!["parse", "beta"]);
//     }
// }
