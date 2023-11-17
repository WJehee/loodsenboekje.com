use serde::{Serialize, Deserialize};
use leptos::*;
use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use super::{db, user::get_user_by_username};
    use crate::auth::user;
    use sqlx::FromRow;
}}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Entry {
    pub id: i64,
    pub how: String,
    pub created: chrono::NaiveDateTime,
}

#[server(AddEntry)]
pub async fn add_entry(how: String, who: String) -> Result<i64, ServerFnError> {
    // check if user has permission to add
    let user = user()?;
    match user.is_writer {
        true => {
            let db = db().await;

            let id = sqlx::query!("INSERT INTO entries (how) VALUES (?)", how)
                .execute(&db)
                .await?
                .last_insert_rowid();
            println!("{user} added entry: {how}");

            for maybe_username in who.split(",") {
                // TODO: if user does not exist, make one, without password so they can still register
                let entry_user = get_user_by_username(maybe_username.trim()).await?;
                sqlx::query!("INSERT INTO user_entries (user_id, entry_id) VALUES (?, ?)", entry_user.id, id)
                    .execute(&db)
                    .await?;
                println!("added {} as author for entry", entry_user.username);
            }
            Ok(id)
        },
        false => {
            println!("{user} does not have permission to add a new entry");
            Err(ServerFnError::ServerError("Invalid permission".into()))
        }
    }

}

#[server]
pub async fn get_entry(id: i64) -> Result<Entry, ServerFnError> {
    user()?;
    let db = db().await;
    let result = sqlx::query_as!(Entry, "SELECT * FROM entries WHERE id = ?", id)
        .fetch_one(&db)
        .await?;
    Ok(result)
}

#[server]
pub async fn get_entries(query: String) -> Result<Vec<Entry>, ServerFnError> {
    user()?;
    let db = db().await;

    // TODO: check this, pretty sure this is secure, as the sql query is still prepared
    let query = format!("%{query}%");
    let result = sqlx::query_as!(Entry, "SELECT * FROM entries WHERE how LIKE ?", query)
        .fetch_all(&db)
        .await?;
    Ok(result)
}

#[server(DeleteEntry)]
pub async fn delete_entry(id: i64) -> Result<(), ServerFnError> {
    let user = user()?;
    match user.is_writer {
        true => {
            let db = db().await;
            sqlx::query!("DELETE FROM entries WHERE id = ?", id)
                .execute(&db)
                .await?;
            println!("{} ({}) deleted entry with id: {id}", user.name, user.id);
            Ok(())
        },
        false => {
            println!("Invalid permission to delete entry for {} ({})", user.name, user.id);
            Err(ServerFnError::ServerError("Invalid permission".into()))
        }
    }
}

