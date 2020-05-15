use category::Category;
use limit::Limit;
use sort_by::SortBy;
use format::Format;
use api_parameters::ApiParameters;

#[derive(Clone, Debug)]
pub struct ApiParametersBuilder {
    ranked: bool,
    limit: Limit,
    categories: Option<Vec<Category>>,
    sort_by: SortBy,
    minimum_seeders: Option<usize>,
    minimum_leechers: Option<usize>,
    format: Format,
}

impl ApiParametersBuilder {
    pub fn ranked(&mut self, ranked: bool) -> &mut ApiParametersBuilder {
        self.ranked = ranked;
        self
    }

    pub fn limit(&mut self, limit: Limit) -> &mut ApiParametersBuilder {
        self.limit = limit;
        self
    }

    pub fn categories(&mut self, categories: Vec<Category>) -> &mut ApiParametersBuilder {
        self.categories = Some(categories);
        self
    }

    pub fn sort_by(&mut self, sort_by: SortBy) -> &mut ApiParametersBuilder {
        self.sort_by = sort_by;
        self
    }

    pub fn minimum_seeders(&mut self, minimum_seeders: usize) -> &mut ApiParametersBuilder {
        self.minimum_seeders = Some(minimum_seeders);
        self
    }

    pub fn minimum_leechers(&mut self, minimum_leechers: usize) -> &mut ApiParametersBuilder {
        self.minimum_leechers = Some(minimum_leechers);
        self
    }

    pub fn format(&mut self, format: Format) -> &mut ApiParametersBuilder {
        self.format = format;
        self
    }

    pub fn new() -> Self {
        ApiParametersBuilder {
            ranked: true,
            limit: Limit::default(),
            categories: None,
            sort_by: SortBy::default(),
            minimum_seeders: None,
            minimum_leechers: None,
            format: Format::default(),
        }
    }

    pub fn build(&self) -> ApiParameters {
        ApiParameters {
            ranked: self.ranked,
            limit: self.limit,
            categories: self.categories.clone(),
            sort_by: self.sort_by,
            minimum_seeders: self.minimum_seeders,
            minimum_leechers: self.minimum_leechers,
            format: self.format,
        }
    }
}
