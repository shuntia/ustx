use serde::{Deserialize, Serialize};

/// Represents the type of an expression.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ExpressionType {
    /// A numerical expression, which can be represented by a single value.
    #[default]
    Numerical,
    /// An options expression, which can be one of a list of options.
    Options,
    /// A curve expression, which is represented by a curve.
    Curve,
}

/// Represents a descriptor for an expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExpressionDescriptor {
    /// The name of the expression.
    pub name: String,
    /// The abbreviation of the expression.
    pub abbr: String,
    /// The type of the expression.
    #[serde(default)]
    pub r#type: ExpressionType,
    /// The minimum value of the expression.
    #[serde(default)]
    pub min: f32,
    /// The maximum value of the expression.
    #[serde(default)]
    pub max: f32,
    /// The default value of the expression.
    #[serde(default)]
    pub default_value: f32,
    /// Whether the expression is a flag.
    #[serde(default)]
    pub is_flag: bool,
    /// The flag of the expression.
    #[serde(default)]
    pub flag: Option<String>,
    /// A list of options for the expression.
    #[serde(default)]
    pub options: Vec<String>,
}

/// Represents an instance of an expression.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Expression {
    /// The index of the expression.
    #[serde(default)]
    pub index: Option<i32>,
    /// The abbreviation of the expression.
    #[serde(default)]
    pub abbr: String,
    /// The value of the expression.
    #[serde(default)]
    pub value: f32,
}

/// Represents a curve.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Curve {
    /// The abbreviation of the curve.
    #[serde(default)]
    pub abbr: String,
    /// The x-coordinates of the curve points.
    #[serde(default)]
    pub xs: Vec<i32>,
    /// The y-coordinates of the curve points.
    #[serde(default)]
    pub ys: Vec<i32>,
}
