use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub is_shared: bool,
    pub shared_with: Vec<String>,
    pub version: u64,
}

impl Note {
    pub fn new(title: String, content: String, user_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            title,
            content,
            created_at: now,
            updated_at: now,
            created_by: user_id,
            is_shared: false,
            shared_with: vec![],
            version: 1,
        }
    }

    pub fn update(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.updated_at = Utc::now();
        self.version += 1;
    }
}
