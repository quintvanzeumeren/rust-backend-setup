

pub struct PasetoClaims;

impl PasetoClaims {
    /// TOKEN_IDENTIFIER is the reserved key used to store the 'token identifier' property within a PASETO token.
    pub const TOKEN_IDENTIFIER: &'static str = "jti";

    /// SUBJECT is the reserved key used to store the 'subject' property within a PASETO token.
    pub const SUBJECT: &'static str = "sub";

    /// AUDIENCE is the reserved key used to store the 'audience' property within a PASETO token.
    pub const AUDIENCE: &'static str = "aud";

    /// ISSUED_AT is the reserved key used to store the 'issued at time' property within a PASETO token.
    pub const ISSUER: &'static str = "iss";

    /// EXPIRATION is the reserved key used to store the 'expiration time' property within a PASETO token.
    pub const EXPIRATION: &'static str = "exp";

    /// NOT_BEFORE is the reserved key used to store the 'not before time' property within a PASETO token.
    pub const NOT_BEFORE: &'static str = "nbf";

    /// ISSUED_AT is the reserved key used to store the 'issued at time' property within a PASETO token.
    pub const ISSUED_AT: &'static str = "iat";

    /// CUSTOM_CLAIMS is the key we use to append custom claims
    pub const CUSTOM_CLAIMS: &'static str = "data";
}