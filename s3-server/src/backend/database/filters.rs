#[derive(Debug, Clone, Default)]
pub struct QueryFilters {
    pub prefix: Option<String>,
    pub limit: Option<i64>,
    pub marker: Option<String>,
    pub delimiter: Option<String>,
}

impl QueryFilters {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
