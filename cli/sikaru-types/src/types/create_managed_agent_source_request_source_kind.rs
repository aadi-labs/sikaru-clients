pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateManagedAgentSourceRequestSourceKind {
    #[serde(rename = "agent_document")]
    AgentDocument,
}
impl fmt::Display for CreateManagedAgentSourceRequestSourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AgentDocument => "agent_document",
        };
        write!(f, "{}", s)
    }
}
