use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::Account;
use crate::repositories::Accounts;

pub struct AccountsImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Accounts for AccountsImpl<'a> {
    async fn find_by(&self, user_name: &str) -> Option<Account> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM accounts WHERE name = $1::TEXT", &[&user_name])
            .await
            .unwrap();
        row.map(|r| r.into())
    }

    async fn store(&self, entity: &Account) -> Result<u64, tokio_postgres::Error> {
        let conn = self.pool.get().await.unwrap();
        conn.execute(
            "INSERT INTO accounts (name, password) VALUES ($1, $2)",
            &[&entity.user_name, &entity.hashed_password]
        )
        .await
    }
}

impl From<Row> for Account {
    fn from(r: Row) -> Self {
        Account::new(
            r.get("id"),
            r.get("name"),
            r.get("password"),
        )
    }
}


