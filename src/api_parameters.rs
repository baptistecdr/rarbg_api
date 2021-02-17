use crate::category::Category;
use crate::format::Format;
use crate::sort_by::SortBy;
use crate::limit::Limit;

#[derive(Clone, Debug)]
pub struct ApiParameters {
    pub(crate) ranked: bool,
    pub(crate) limit: Limit,
    pub(crate) categories: Option<Vec<Category>>,
    pub(crate) sort_by: SortBy,
    pub(crate) minimum_seeders: Option<u32>,
    pub(crate) minimum_leechers: Option<u32>,
    pub(crate) format: Format,
}

impl ApiParameters {
    /// Return true if the request will show only the scene releases + rarbg releases
    /// + rartv releases, otherwise false, the request will show all releases.
    pub fn ranked(&self) -> &bool {
        &self.ranked
    }

    /// Return the number of torrents that the request will return.
    pub fn limit(&self) -> &Limit {
        &self.limit
    }

    /// Return in which categories the request will operate.
    pub fn categories(&self) -> Option<&Vec<Category>> {
        self.categories.as_ref()
    }

    /// Return the sorting criteria.
    pub fn sort_by(&self) -> &SortBy { &self.sort_by }

    /// Return the number of minimum seeders that a torrent will have.
    pub fn minimum_seeders(&self) -> Option<&u32> {
        self.minimum_seeders.as_ref()
    }

    /// Return the number of minimum leechers that a torrent will have.
    pub fn minimum_leechers(&self) -> Option<&u32> {
        self.minimum_leechers.as_ref()
    }

    /// Return the response format of the request.
    pub fn format(&self) -> &Format {
        &self.format
    }
}
