use crate::cli::JwtSignOpts;
use anyhow::Ok;
use jsonwebtoken::{DecodingKey, EncodingKey, TokenData, Validation};

pub fn jwt_sign_by_hs256(
    header: &jsonwebtoken::Header,
    clam: &JwtSignOpts,
    secret: &str,
) -> anyhow::Result<String> {
    let token = jsonwebtoken::encode(header, clam, &EncodingKey::from_secret(secret.as_ref()))?;

    Ok(token)
}

pub fn jwt_verify_by_hs256(token: &str) -> anyhow::Result<TokenData<JwtSignOpts>> {
    let decoded_data = jsonwebtoken::decode::<JwtSignOpts>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?;

    Ok(decoded_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::Header;

    #[test]
    fn test_jwt_sign() -> anyhow::Result<()> {
        let header = Header::default();
        let payload = JwtSignOpts {
            sub: "b@b.com".into(),
            company: "ACME".into(),
            exp: 10000000000,
        };
        let token = jwt_sign_by_hs256(&header, &payload, "secret")?;

        assert_eq!(token, "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM".to_string());
        Ok(())
    }

    #[test]
    fn test_jwt_verify() -> anyhow::Result<()> {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM".to_string();
        let decoded_data = jwt_verify_by_hs256(&token)?;
        assert_eq!(decoded_data.claims.sub, "b@b.com");
        assert_eq!(decoded_data.claims.company, "ACME");
        assert_eq!(decoded_data.claims.exp, 10000000000);
        Ok(())
    }
}
