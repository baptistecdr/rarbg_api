use crate::api_parameters::ApiParameters;
use crate::category::Category;
use crate::format::Format;
use crate::limit::Limit;
use crate::sort_by::SortBy;

#[derive(Clone, Debug)]
pub struct ApiParametersBuilder {
    ranked: bool,
    limit: Limit,
    categories: Option<Vec<Category>>,
    sort_by: SortBy,
    minimum_seeders: Option<u32>,
    minimum_leechers: Option<u32>,
    format: Format,
}

impl ApiParametersBuilder {
    /// Ranked torrents are scene releases + rarbg releases + rartv releases.
    ///
    /// If ranked is false, the API will show all the torrents.
    ///
    /// # Example
    /// ```
    /// use rarbg_api::api_parameters_builder::ApiParametersBuilder;
    ///
    /// let apb = ApiParametersBuilder::new().ranked(false);
    /// ```
    pub fn ranked(&mut self, ranked: bool) -> &mut ApiParametersBuilder {
        self.ranked = ranked;
        self
    }

    /// You can limit the number of torrents to list or search by 25, 50 or 100.
    ///
    /// # Example
    /// ```
    /// use rarbg_api::api_parameters_builder::ApiParametersBuilder;
    /// use rarbg_api::limit::Limit;
    ///
    /// let apb = ApiParametersBuilder::new().limit(Limit::OneHundred);
    /// ```
    pub fn limit(&mut self, limit: Limit) -> &mut ApiParametersBuilder {
        self.limit = limit;
        self
    }

    /// You can filter torrents by categories.
    ///
    /// # Example
    /// ```
    /// use rarbg_api::api_parameters_builder::ApiParametersBuilder;
    /// use rarbg_api::category::Category;
    ///
    /// let apb = ApiParametersBuilder::new().categories(vec![Category::TvHdEpisodes, Category::TvUhdEpisodes]);
    /// ```
    pub fn categories(&mut self, categories: Vec<Category>) -> &mut ApiParametersBuilder {
        self.categories = Some(categories);
        self
    }

    /// You can sort torrents by seeders, leechers or last uploaded.
    ///
    /// # Example
    /// ```
    /// use rarbg_api::api_parameters_builder::ApiParametersBuilder;
    /// use rarbg_api::sort_by::SortBy;
    ///
    /// let apb = ApiParametersBuilder::new().sort_by(SortBy::Seeders);
    /// ```
    pub fn sort_by(&mut self, sort_by: SortBy) -> &mut ApiParametersBuilder {
        self.sort_by = sort_by;
        self
    }

    /// You can specify the minimum number of seeders that a torrent must have.
    ///
    /// # Example
    /// ```
    /// use rarbg_api::api_parameters_builder::ApiParametersBuilder;
    ///
    /// let apb = ApiParametersBuilder::new().minimum_seeders(42);
    /// ```
    pub fn minimum_seeders(&mut self, minimum_seeders: u32) -> &mut ApiParametersBuilder {
        self.minimum_seeders = Some(minimum_seeders);
        self
    }

    /// You can specify the minimum number of leechers that a torrent must have.
    ///
    /// # Example
    /// ```
    /// use rarbg_api::api_parameters_builder::ApiParametersBuilder;
    ///
    /// let apb = ApiParametersBuilder::new().minimum_leechers(42);
    /// ```
    pub fn minimum_leechers(&mut self, minimum_leechers: u32) -> &mut ApiParametersBuilder {
        self.minimum_leechers = Some(minimum_leechers);
        self
    }

    /// You can choose the format of the response in order to have less or more details on each torrent.
    ///
    /// # Example
    /// ```
    /// use rarbg_api::api_parameters_builder::ApiParametersBuilder;
    /// use rarbg_api::format::Format;
    ///
    /// let apb = ApiParametersBuilder::new().format(Format::JsonExtended);
    /// ```
    pub fn format(&mut self, format: Format) -> &mut ApiParametersBuilder {
        self.format = format;
        self
    }

    /// Create a new ApiParametersBuilder with default options.
    ///
    /// Default options are :
    /// * Only ranked torrents
    /// * Limit torrents to 25
    /// * No categories filtering
    /// * Sorted by last uploaded
    /// * No minimum seeders/leechers
    /// * Json format
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

    /// Construct an immutable ApiParameters that can be use with RarBgApi.
    ///
    /// # Example
    /// ```
    /// use rarbg_api::api_parameters_builder::ApiParametersBuilder;
    /// use rarbg_api::api_parameters::ApiParameters;
    /// use rarbg_api::limit::Limit;
    /// use rarbg_api::sort_by::SortBy;
    /// use rarbg_api::format::Format;
    /// use rarbg_api::category::Category;
    ///
    /// let ap :ApiParameters = ApiParametersBuilder::new()
    ///                          .ranked(false)
    ///                          .limit(Limit::OneHundred)
    ///                          .categories(vec![Category::TvHdEpisodes, Category::TvUhdEpisodes])
    ///                          .sort_by(SortBy::Seeders)
    ///                          .minimum_seeders(42)
    ///                          .minimum_leechers(42)
    ///                          .format(Format::JsonExtended)
    ///                          .build();
    /// ```
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
