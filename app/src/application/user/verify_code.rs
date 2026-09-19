//pub struct VerifyCodeUseCase {
//    pub cache: Arc<dyn Cache>,
//    pub user_repo: Arc<dyn UserRepository>,
//}
//
//impl VerifyCodeUseCase {
//    pub async fn execute(&self, phone: String, code: String) -> Result<String> {
//        let key = format!("auth:code:{}", phone);
//
//        let stored = self.cache.get(&key).await?
//            .ok_or(Error::InvalidCode)?;
//
//        if stored != code {
//            return Err(Error::InvalidCode);
//        }
//
//        // delete after success
//        self.cache.delete(&key).await?;
//
//        // find or create user
//        let user = self.user_repo.find_or_create(phone).await?;
//
//        Ok(user.id)
//    }
//}
