pub struct LifecycleRule {
    pub id: String,
    pub prefix: String,
    pub status: String,
    pub expiration: String,
}

pub struct Lifecycle {
    pub rules: Vec<LifecycleRule>,
}
