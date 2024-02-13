use url::Url;
use user::UserAuth;

pub mod err;
pub mod items;
pub mod serde;
pub mod session;
pub mod user;
pub mod utils;

#[derive(Debug, Clone)]
pub struct JellyfinClient {
    url: Url,
    client: reqwest::Client,
    auth: Option<UserAuth>,
}

/// Represents a client for interacting with a Jellyfin server.
///
/// # Examples
///
/// Creating a new client without authentication:
///
/// ```no_run
/// # use jellyfin_rs::JellyfinClient;
/// #
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = JellyfinClient::new("http://example.com").await?;
/// # Ok(())
/// # }
/// ```
///
/// Creating a new client with username and password authentication:
///
/// ```no_run
/// # use jellyfin_rs::JellyfinClient;
/// #
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = JellyfinClient::new_auth_name("http://example.com", "user", "password").await?;
/// # Ok(())
/// # }
/// ```
impl JellyfinClient {
    /// Creates a new instance of `JellyfinClient` without authentication.
    ///
    /// # Parameters
    ///
    /// - `url`: The base URL of the Jellyfin server, without a trailing slash.
    ///
    /// # Returns
    ///
    /// Returns a `Result` wrapping `JellyfinClient` if the URL is valid and the client was successfully created.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid.
    pub async fn new<T: Into<String>>(url: T) -> err::Result<Self> {
        let url_str = url.into();
        let trimmed_url_str = url_str.trim_end_matches('/'); // Remove trailing slash

        Ok(Self {
            url: Url::parse(trimmed_url_str)?,
            client: reqwest::Client::new(),
            auth: None,
        })
    }

    /// Creates a new instance of `JellyfinClient` with standard authentication using a user ID and password.
    ///
    /// # Parameters
    ///
    /// - `url`: The base URL of the Jellyfin server, without a trailing slash.
    /// - `id`: The user ID for authentication.
    /// - `password`: The password for authentication.
    ///
    /// # Returns
    ///
    /// Returns a `Result` wrapping `JellyfinClient` if the URL is valid, and authentication is successful.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid, or authentication fails.
    pub async fn new_auth_std<T: Into<String>>(url: T, id: T, password: T) -> err::Result<Self> {
        let url_str = url.into();
        let trimmed_url_str = url_str.trim_end_matches('/'); // Remove trailing slash

        let mut client = Self {
            url: Url::parse(trimmed_url_str)?,
            client: reqwest::Client::new(),
            auth: None,
        };
        client.auth_user_std(id.into(), password.into()).await?;
        Ok(client)
    }

    /// Creates a new instance of `JellyfinClient` with name-based authentication.
    ///
    /// # Parameters
    ///
    /// - `url`: The base URL of the Jellyfin server, without a trailing slash.
    /// - `username`: The username for authentication.
    /// - `password`: The password for authentication.
    ///
    /// # Returns
    ///
    /// Returns a `Result` wrapping `JellyfinClient` if the URL is valid, and authentication is successful.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid, or authentication fails.
    pub async fn new_auth_name<T: Into<String>>(
        url: T,
        username: T,
        password: T,
    ) -> err::Result<Self> {
        let url_str = url.into();
        let trimmed_url_str = url_str.trim_end_matches('/'); // Remove trailing slash

        let mut client = Self {
            url: Url::parse(trimmed_url_str)?,
            client: reqwest::Client::new(),
            auth: None,
        };
        client
            .auth_user_name(username.into(), password.into())
            .await?;
        Ok(client)
    }
}

#[cfg(test)]
#[path = "tests/lib.rs"]
pub mod tests;
