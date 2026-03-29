use crate::context::oauth::error::DomainError;


#[derive(Debug, Clone, PartialEq)]
pub enum CodeChallengeMethod {
    S256,
    // 传就报错
    Plain,
}

pub struct PkceInfo {
    code_challenge: String,
    code_challenge_method: CodeChallengeMethod
}

impl PkceInfo {
    pub fn new(code_challenge: String, code_challenge_method: CodeChallengeMethod) -> Result<Self, DomainError> {
        if code_challenge_method == CodeChallengeMethod::Plain {
            return Err(DomainError::UnsupportedCodeChallengeMethod);
        }

        Ok(Self { code_challenge, code_challenge_method })
    }

    pub fn verify(&self, code_verifier: String) -> Result<bool, DomainError> {
        // 校验逻辑是：Base64Url(method(code_verifier)) == code_challenge
        match self.code_challenge_method {
            CodeChallengeMethod::S256 => {
                use sha2::{Sha256, Digest};
                use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

                let mut hasher = Sha256::new();
                hasher.update(code_verifier.as_bytes());
                let hash = hasher.finalize();

                let encoded = URL_SAFE_NO_PAD.encode(hash);
                Ok(encoded == self.code_challenge)
            }
            CodeChallengeMethod::Plain => {
                Err(DomainError::UnsupportedCodeChallengeMethod)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkce_verify_rfc_7636_example() {
        // RFC 7636 Appendix B Example
        // code_verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"
        // code_challenge = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
        
        let pkce = PkceInfo::new(
            "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM".to_string(),
            CodeChallengeMethod::S256,
        ).unwrap();
        
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk".to_string();
        
        let result = pkce.verify(verifier).unwrap();
        assert!(result);
    }

    #[test]
    fn test_pkce_verify_fail() {
        let pkce = PkceInfo::new(
            "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM".to_string(),
            CodeChallengeMethod::S256,
        ).unwrap();
        
        let wrong_verifier = "wrong_verifier".to_string();
        
        assert!(!pkce.verify(wrong_verifier).unwrap());
    }
}
