pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TranscriptStreamEvent {
        TranscriptEvent(TranscriptEvent),

        OutputPreview(OutputPreview),
}

impl TranscriptStreamEvent {
    pub fn is_transcript_event(&self) -> bool {
        matches!(self, Self::TranscriptEvent(_))
    }

    pub fn is_output_preview(&self) -> bool {
        matches!(self, Self::OutputPreview(_))
    }


    pub fn as_transcript_event(&self) -> Option<&TranscriptEvent> {
        match self {
                    Self::TranscriptEvent(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_transcript_event(self) -> Option<TranscriptEvent> {
        match self {
                    Self::TranscriptEvent(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_output_preview(&self) -> Option<&OutputPreview> {
        match self {
                    Self::OutputPreview(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_output_preview(self) -> Option<OutputPreview> {
        match self {
                    Self::OutputPreview(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TranscriptStreamEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TranscriptEvent(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::OutputPreview(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
