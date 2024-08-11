pub struct Cors {
    pub cors_rules: Vec<CorsRule>,
}

pub struct CorsRule {
    pub id: String,
    pub allowed_methods: Vec<String>,
    pub allowed_origins: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age_seconds: i64,
    pub expose_headers: Vec<String>,
}
