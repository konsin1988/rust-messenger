//use std::sync::Arc;

//pub struct StartAuthUseCase {
//    pub cache: Arc<dyn Cache>,      // Redis
//    pub sms: Arc<dyn SmsProvider>,  // SMS service
//}
//
//impl StartAuthUseCase {
//    pub async fn execute(&self, phone: String) -> Result<()> {
//        let code = format!("{:04}", rand::random::<u16>() % 10000);
//
//        self.cache
//            .set_with_ttl(format!("auth:code:{}", phone), code.clone(), 300)
//            .await?;
//
//        self.sms.send(phone, format!("Your code: {}", code)).await?;
//
//        Ok(())
//    }
//}
