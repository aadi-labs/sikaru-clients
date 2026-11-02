pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum DecisionAnswersValue {
        #[serde(rename = "choice")]
        #[non_exhaustive]
        Choice {
            #[serde(default)]
            choice: String,
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            confidence: f64,
            #[serde(default)]
            probabilities: HashMap<String, f64>,
        },

        #[serde(rename = "noul")]
        #[non_exhaustive]
        Noul {
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            noul: f64,
        },

        #[serde(rename = "score")]
        #[non_exhaustive]
        Score {
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            confidence: f64,
            #[serde(default)]
            legend: HashMap<String, String>,
            #[serde(default)]
            probabilities: HashMap<String, f64>,
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            score: f64,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl DecisionAnswersValue {
    pub fn choice(choice: String, confidence: f64, probabilities: HashMap<String, f64>) -> Self {
        Self::Choice { choice, confidence, probabilities }
    }

    pub fn noul(noul: f64) -> Self {
        Self::Noul { noul }
    }

    pub fn score(confidence: f64, legend: HashMap<String, String>, probabilities: HashMap<String, f64>, score: f64) -> Self {
        Self::Score { confidence, legend, probabilities, score }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
