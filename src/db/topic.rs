use crate::{error::AppError, form::CreateTopic, model::TopicID, Result};
use deadpool_postgres::Client;

pub async fn create(client: &mut Client, ct: &CreateTopic) -> Result<TopicID> {
    let tx = client.transaction().await.map_err(AppError::from)?;
    let stmt = tx.prepare("").await;
    let stmt = match stmt {
        Ok(s) => s,
        Err(err) => {
            tx.rollback().await.map_err(AppError::from)?;
            return Err(AppError::from(err));
        }
    };
    tx.commit().await.map_err(AppError::from)?;
    Ok(TopicID { id: 1 })
}
