use serde::Deserialize;

use super::client::{
    API_BASE,
    TidalClient,
};
use super::models::{
    Folder,
    FolderItem,
    ItemsPage,
    SessionInfo,
    Subscription,
    UserProfile,
};
use crate::core::error::Result;

impl TidalClient {
    pub async fn get_session(&mut self) -> Result<SessionInfo> {
        let session: SessionInfo = self.get(&format!("{}/sessions", API_BASE)).await?;
        self.country_code = session.country_code.clone();
        self.user_id = Some(session.user_id);
        Ok(session)
    }

    pub async fn get_user(&self, user_id: u64) -> Result<UserProfile> {
        let url = self.api_url(&format!("users/{}", user_id), &[]);
        self.get(&url).await
    }

    pub async fn get_subscription(&self, user_id: u64) -> Result<Subscription> {
        let url = self.api_url(&format!("users/{}/subscription", user_id), &[]);
        self.get(&url).await
    }

    pub async fn get_folders(&self, user_id: u64) -> Result<Vec<Folder>> {
        let url = self.api_url(&format!("users/{}/folders", user_id), &[]);
        #[derive(Deserialize)]
        struct FoldersResponse {
            items: Vec<Folder>,
        }
        let resp: FoldersResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_folder_items(
        &self,
        user_id: u64,
        folder_id: &str,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<FolderItem>> {
        let url = self.api_url(
            &format!("users/{}/folders/{}/items", user_id, folder_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn create_folder(
        &self,
        user_id: u64,
        name: &str,
        parent: Option<&str>,
    ) -> Result<Folder> {
        let url = self.api_url(&format!("users/{}/folders", user_id), &[]);
        let mut body = serde_json::json!({ "name": name });
        if let Some(p) = parent {
            body["parent"] = serde_json::json!(p);
        }
        self.post(&url, Some(&body.to_string())).await
    }

    pub async fn delete_folder(&self, user_id: u64, folder_id: &str) -> Result<()> {
        let url = self.api_url(&format!("users/{}/folders/{}", user_id, folder_id), &[]);
        self.delete_empty(&url).await
    }
}
