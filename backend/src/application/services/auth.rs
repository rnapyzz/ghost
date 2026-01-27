use crate::{
    domain::user::{User, UserRepository},
    utils::password,
};

pub struct AuthService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn signup(
        &self,
        name: String,
        email: String,
        raw_password: String,
    ) -> anyhow::Result<User> {
        // emailの重複チェック
        if self.repository.find_by_email(&email).await?.is_some() {
            return Err(anyhow::anyhow!("Email already exists"));
        }

        let hash = password::hash(&raw_password)?;
        let new_user = User::new(name, email, hash)?;
        let created_user = self.repository.create(&new_user).await?;

        Ok(created_user)
    }
}
