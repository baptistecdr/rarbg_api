use category::Category;
use limit::Limit;
use sort_by::SortBy;
use format::Format;

#[derive(Clone, Debug)]
pub struct ApiParameters {
    pub(crate) ranked: bool,
    pub(crate) limit: Limit,
    pub(crate) categories: Option<Vec<Category>>,
    pub(crate) sort_by: SortBy,
    pub(crate) minimum_seeders: Option<usize>,
    pub(crate) minimum_leechers: Option<usize>,
    pub(crate) format: Format,
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
}
