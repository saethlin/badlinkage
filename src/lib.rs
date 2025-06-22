use aws_lc_rs::hmac::Algorithm;

pub fn init() -> Vec<fn()> {
    vec![sign, generate]
}

fn sign() {
    let arg: Option<CryptoHash> = std::hint::black_box(None);
    let hash: Algorithm = arg.unwrap().into();
    std::hint::black_box(hash);
}

fn generate() {
    let arg = std::hint::black_box(CryptoHash::Sha1);
    let hash = match arg {
        CryptoHash::Sha1 => &aws_lc_rs::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
        CryptoHash::Sha256 => &aws_lc_rs::hmac::HMAC_SHA256,
        CryptoHash::Sha384 => &aws_lc_rs::hmac::HMAC_SHA384,
    };
    std::hint::black_box(hash.digest_algorithm());
}

enum CryptoHash {
    Sha1,
    Sha256,
    Sha384,
}
impl From<CryptoHash> for Algorithm {
    fn from(hash: CryptoHash) -> Algorithm {
        match hash {
            CryptoHash::Sha1 => aws_lc_rs::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            CryptoHash::Sha256 => aws_lc_rs::hmac::HMAC_SHA256,
            CryptoHash::Sha384 => aws_lc_rs::hmac::HMAC_SHA384,
        }
    }
}
