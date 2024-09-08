use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::prelude::*;

#[derive(Debug, Model, Serialize, Deserialize)]
#[model(index(
    keys = r#"doc!{"message_id": 1, "chat_id": 1}"#,
    options = r#"doc!{"unique": true}"#
))]
struct User {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The user's email address.
    pub message_id: String,
    pub chat_id: String,
}
