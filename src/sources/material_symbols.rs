use reqwest::Url;
use serde::Deserialize;

const MATERIAL_SYMBOLS_URL: &str =
    "https://raw.githubusercontent.com/google/material-design-icons/refs/heads/master/symbols/web";

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
pub enum MaterialSymbolsSize {
    #[serde(rename = "20px")]
    Size20px,
    #[serde(rename = "24px")]
    Size24px,
    #[serde(rename = "40px")]
    Size40px,
    #[serde(rename = "48px")]
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

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MaterialSymbolsWeight {
    #[serde(rename = "100")]
    Weight100,
    #[serde(rename = "200")]
    Weight200,
    #[serde(rename = "300")]
    Weight300,
    #[serde(rename = "400")]
    #[default]
    Weight400,
    #[serde(rename = "500")]
    Weight500,
    #[serde(rename = "600")]
    Weight600,
    #[serde(rename = "700")]
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

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum MaterialSymbolsGrade {
    #[serde(rename = "-25")]
    GradeNegative25 = -25,
    #[serde(rename = "0")]
    #[default]
    Grade0 = 0,
    #[serde(rename = "200")]
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

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct MaterialSymbolsSource {
    #[serde(rename = "material_symbols")]
    symbol: String,
    #[serde(default)]
    style: MaterialSymbolsStyle,
    #[serde(default)]
    variant: MaterialSymbolsVariant,
    #[serde(default)]
    weight: MaterialSymbolsWeight,
    #[serde(default)]
    grade: MaterialSymbolsGrade,
    #[serde(default)]
    size: MaterialSymbolsSize,
}

impl MaterialSymbolsSource {
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

    pub async fn fetch(&self, reqwest: reqwest::Client) -> anyhow::Result<()> {
        let url = self.url()?;

        println!("{}", url);

        let response = reqwest
            .get(url)
            .header("cache-control", "public, max-age=3600")
            .send()
            .await?;

        println!("{}", response.text().await?);

        Ok(())
    }
}
