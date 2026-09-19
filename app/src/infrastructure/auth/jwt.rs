use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

const SECRET: &[u8] = b"super_secret";

pub fn create_jwt(user_id: &str) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now().timestamp() + 3600) as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
        .unwrap()
}

pub fn verify_jwt(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .ok()
}
