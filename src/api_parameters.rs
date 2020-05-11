use category::Category;
use limit::Limit;
use sort_by::SortBy;
use format::Format;

#[derive(Clone, Debug)]
pub struct ApiParameters {
    ranked: bool,
    limit: Limit,
    categories: Option<Vec<Category>>,
    sort_by: SortBy,
    minimum_seeders: Option<usize>,
    minimum_leechers: Option<usize>,
    format: Format,
}

impl ApiParameters {
    pub fn ranked(&self) -> &bool {
        &self.ranked
    }

    pub fn limit(&self) -> &Limit {
        &self.limit
    }

    pub fn categories(&self) -> &Option<Vec<Category>> {
        &self.categories
    }

    pub fn sort_by(&self) -> &SortBy { &self.sort_by }

    pub fn minimum_seeders(&self) -> &Option<usize> {
        &self.minimum_seeders
    }

    pub fn minimum_leechers(&self) -> &Option<usize> {
        &self.minimum_leechers
    }

    pub fn format(&self) -> &Format {
        &self.format
    }

    pub fn new(ranked: Option<bool>, limit: Option<Limit>, categories: Option<Vec<Category>>, sort_by: Option<SortBy>, minimum_seeders: Option<usize>, minimum_leechers: Option<usize>, format: Option<Format>) -> Self {
        ApiParameters {
            ranked: ranked.unwrap_or_default(),
            limit: limit.unwrap_or_default(),
            categories,
            sort_by: sort_by.unwrap_or_default(),
            minimum_seeders,
            minimum_leechers,
            format: format.unwrap_or_default()
        }
    }
}
