use crate::errors::ServiceError;
use crate::utils::crypto::pwhash;
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
        let result = db_conn.execute(
            "INSERT INTO users (email, hash_pass) VALUES (?1, ?2)",
            (&self.email, &self.hash_pass),
        );
        let Ok(updated) = result else {
            return Err(result.err().unwrap().into());
        };
        if updated == 0 {
            return Err(ServiceError::AlreadyExists(anyhow::anyhow!(
                "AlreadyExists"
            )));
        }
        if updated > 1 {
            return Err(ServiceError::AlreadyExists(anyhow::anyhow!(
                "Multiple records with same email exist"
            )));
        }
        let result = db_conn
            .prepare("SELECT id, created_at, email, hash_pass FROM users where email = :email");
        let Ok(mut stmt) = result else {
            return Err(result.err().unwrap().into());
        };
        let result = stmt.query_map(&[(":email", &self.email)], |row| {
            Ok(User {
                id: row.get(0)?,
                created_at: row.get(1)?,
                email: row.get(2)?,
                hash_pass: row.get(3)?,
            })
        });
        let Ok(mut user_iter) = result else {
            return Err(result.err().unwrap().into());
        };
        if let Some(u) = user_iter.next() {
            if let Ok(user) = u {
                return Ok(user.clone());
            } else {
                return Err(ServiceError::Unauthorized);
            }
        }
        return Err(ServiceError::Unauthorized);
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
