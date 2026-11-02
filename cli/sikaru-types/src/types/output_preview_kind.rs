pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Set for code execution output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OutputPreviewKind {
    #[serde(rename = "execution")]
    Execution,
}
impl fmt::Display for OutputPreviewKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Execution => "execution",
        };
        write!(f, "{}", s)
    }
}
