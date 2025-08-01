use anyhow::Context;
use reqwest::Url;
use serde::Deserialize;

use crate::sources::SpriteSource;

const MATERIAL_SYMBOLS_URL: &str =
    "https://raw.githubusercontent.com/google/material-design-icons/refs/heads/master/symbols/web";

const PATH: &str = "<path ";
const PATH_REPLACE_WITH_WHITE_FILL: &str = "<path fill=\"#fff\" ";

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum MaterialSymbolsStyle {
    #[default]
    Outlined,
    Rounded,
    Sharp,
}

impl MaterialSymbolsStyle {
    pub fn as_dir_name(&self) -> &str {
        match self {
            Self::Outlined => "materialsymbolsoutlined",
            Self::Rounded => "materialsymbolsrounded",
            Self::Sharp => "materialsymbolssharp",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum MaterialSymbolsVariant {
    Filled,
    #[default]
    Outlined,
}

impl MaterialSymbolsVariant {
    pub fn as_file_name_segment(&self) -> &str {
        match self {
            Self::Filled => "fill1",
            Self::Outlined => "",
        }
    }
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
pub enum MaterialSymbolsSize {
    #[cfg_attr(feature = "serde", serde(rename = "20px"))]
    Size20px,
    #[cfg_attr(feature = "serde", serde(rename = "24px"))]
    Size24px,
    #[cfg_attr(feature = "serde", serde(rename = "40px"))]
    Size40px,
    #[cfg_attr(feature = "serde", serde(rename = "48px"))]
    #[default]
    Size48px,
}

impl MaterialSymbolsSize {
    pub fn as_file_name_segment(&self) -> &str {
        match self {
            Self::Size20px => "20px",
            Self::Size24px => "24px",
            Self::Size40px => "40px",
            Self::Size48px => "48px",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum MaterialSymbolsWeight {
    #[cfg_attr(feature = "serde", serde(rename = "100"))]
    Weight100,
    #[cfg_attr(feature = "serde", serde(rename = "200"))]
    Weight200,
    #[cfg_attr(feature = "serde", serde(rename = "300"))]
    Weight300,
    #[cfg_attr(feature = "serde", serde(rename = "400"))]
    #[default]
    Weight400,
    #[cfg_attr(feature = "serde", serde(rename = "500"))]
    Weight500,
    #[cfg_attr(feature = "serde", serde(rename = "600"))]
    Weight600,
    #[cfg_attr(feature = "serde", serde(rename = "700"))]
    Weight700,
}

impl MaterialSymbolsWeight {
    pub fn as_file_name_segment(&self) -> &str {
        match self {
            Self::Weight100 => "wght100",
            Self::Weight200 => "wght200",
            Self::Weight300 => "wght300",
            Self::Weight400 => "",
            Self::Weight500 => "wght500",
            Self::Weight600 => "wght600",
            Self::Weight700 => "wght700",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum MaterialSymbolsGrade {
    #[cfg_attr(feature = "serde", serde(rename = "-25"))]
    GradeNegative25 = -25,
    #[cfg_attr(feature = "serde", serde(rename = "0"))]
    #[default]
    Grade0 = 0,
    #[cfg_attr(feature = "serde", serde(rename = "200"))]
    Grade200 = 200,
}

impl MaterialSymbolsGrade {
    pub fn as_file_name_segment(&self) -> &str {
        match self {
            Self::GradeNegative25 => "gradN25",
            Self::Grade0 => "",
            Self::Grade200 => "grad200",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct MaterialSymbolsSource {
    #[cfg_attr(feature = "serde", serde(rename = "material_symbols"))]
    symbol: String,
    #[cfg_attr(feature = "serde", serde(default))]
    style: MaterialSymbolsStyle,
    #[cfg_attr(feature = "serde", serde(default))]
    variant: MaterialSymbolsVariant,
    #[cfg_attr(feature = "serde", serde(default))]
    weight: MaterialSymbolsWeight,
    #[cfg_attr(feature = "serde", serde(default))]
    grade: MaterialSymbolsGrade,
    #[cfg_attr(feature = "serde", serde(default))]
    size: MaterialSymbolsSize,
}

impl MaterialSymbolsSource {
    #[must_use]
    pub fn file_name(&self) -> String {
        let inner = format!(
            "{}{}{}",
            self.weight.as_file_name_segment(),
            self.grade.as_file_name_segment(),
            self.variant.as_file_name_segment(),
        );

        if inner.is_empty() {
            return format!(
                "{}_{}.svg",
                self.symbol.clone(),
                self.size.as_file_name_segment()
            );
        }

        format!(
            "{}_{}_{}.svg",
            self.symbol.clone(),
            inner,
            self.size.as_file_name_segment()
        )
    }

    pub fn url(&self) -> anyhow::Result<Url> {
        Ok(Url::parse(
            format!(
                "{}/{}/{}/{}",
                MATERIAL_SYMBOLS_URL,
                self.symbol.clone(),
                self.style.as_dir_name(),
                self.file_name()
            )
            .as_str(),
        )?)
    }

    pub async fn fetch(&self, reqwest: reqwest::Client) -> anyhow::Result<SpriteSource> {
        let url = self.url()?;

        let svg = reqwest
            .get(url)
            .header("cache-control", "public, max-age=3600")
            .send()
            .await
            .context("failed to fetch material symbols")?
            .text()
            .await
            .context("failed to parse material symbols as text")?
            .replace(PATH, PATH_REPLACE_WITH_WHITE_FILL);

        let tree = usvg::Tree::from_str(svg.as_str(), &usvg::Options::default())
            .context("failed to parse material symbols as svg")?;

        Ok(SpriteSource::Tree(tree))
    }
}
