use crate::{
    domain::user::{User, UserRepository},
    utils::{jwt, password},
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

        // パスワードのハッシュ化
        let hash = password::hash(&raw_password)?;

        // ユーザーインスタンスを生成
        let new_user = User::new(name, email, hash)?;

        // ユーザーを作成
        let created_user = self.repository.create(&new_user).await?;

        Ok(created_user)
    }

    pub async fn login(&self, email: String, password: String) -> anyhow::Result<String> {
        let user = self
            .repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid mail or password"))?;

        let stored_hash = user
            .password_hash
            .ok_or_else(|| anyhow::anyhow!("This user must login via SSO"))?;

        if !password::verify(&password, &stored_hash)? {
            return Err(anyhow::anyhow!("Invalid email or password"));
        }

        let token = jwt::generate_token(user.id, user.role)?;

        Ok(token)
    }
}
