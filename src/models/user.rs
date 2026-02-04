use crate::errors::ServiceError;
use crate::utils::crypto::pwhash;
use chrono::Local;
use chrono::NaiveDateTime;
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub created_at: NaiveDateTime, // Local::now().naive_local()
    //
    pub email: String,
    pub hash_pass: String,
}

#[derive(Debug)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub hash_pass: String,
}
impl<'a> NewUser<'a> {
    pub fn from_credentials(email: &'a str, password: &'a str) -> Result<Self, ServiceError> {
        let hash_pass = pwhash::hash_password(password)?;
        Ok(Self { email, hash_pass })
    }
    pub fn insert(
        self,
        db_conn: &r2d2::PooledConnection<SqliteConnectionManager>,
    ) -> Result<User, ServiceError> {
        match db_conn.execute(
            "INSERT INTO user (created_at, email, hash_pass) VALUES (?1, ?2, ?3)",
            (Local::now().naive_local(), &self.email, &self.hash_pass),
        ) {
            Ok(updated) => println!("{} rows were updated", updated),
            Err(err) => return Err(err.into()),
        }
        let mut stmt = db_conn
            .prepare("SELECT id, created_at, email, hash_pass FROM user")
            .or(Err(ServiceError::Unauthorized))?;
        let user_iter = stmt
            .query_map([], |row| {
                Ok(User {
                    id: row.get(0)?,
                    created_at: row.get(1)?,
                    email: row.get(2)?,
                    hash_pass: row.get(3)?,
                })
            })
            .or(Err(ServiceError::Unauthorized))?;

        if let Some(u) = user_iter
            .collect::<Vec<Result<User, rusqlite::Error>>>()
            .first()
        {
            if let Ok(user) = u {
                return Ok(user.clone());
            } else {
                return Err(ServiceError::Unauthorized);
            }
        } else {
            return Err(ServiceError::Unauthorized);
        }
    }
}

/// UserResp represents a User as gets returned by the API
#[derive(Serialize, Deserialize)]
pub struct UserResp {
    pub user: UserPub,
}
/// UserPub is a User stripped to essential, public fields
#[derive(Serialize, Deserialize)]
pub struct UserPub {
    pub id: i64,
    pub email: String,
}
impl From<User> for UserResp {
    fn from(user: User) -> Self {
        UserResp {
            user: UserPub {
                id: user.id,
                email: user.email,
            },
        }
    }
}
