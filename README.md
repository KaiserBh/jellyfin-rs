# Project Status

`jellyfin-rs` is currently in alpha stage. This means that while it's functional, the library does not yet cover all Jellyfin API endpoints, and the implementation of some features may change. We welcome contributions and feedback to help expand the library's capabilities and improve its stability. Please use it with an understanding that, as an alpha version, changes and improvements are ongoing.

# Jellyfin-rs

`jellyfin-rs` is an asynchronous Rust client for interacting with the Jellyfin media server. It offers a straightforward way to connect to Jellyfin servers, handle authentication, and perform various operations against the server's API. This library is designed for Rust applications that require communication with Jellyfin, providing both authenticated and non-authenticated access to the server functionalities.

## Features

- Asynchronous API calls using reqwest and tokio.
- Supports non-authenticated and authenticated sessions.
- Easy to use with Rust's powerful async/await syntax.
- Supports username/password and user ID/password authentication methods.

## Getting Started

To use jellyfin-rs in your project, add it as a dependency in your Cargo.toml:

```toml
[dependencies]
jellyfin-rs = "0.1.0"
reqwest = "0.11"
tokio = { version = "1", features = ["full"] }
url = "2.2"
```

### Creating a Client

You can create a `JellyfinClient` instance in several ways depending on your authentication needs:

Without Authentication

```rust
use jellyfin_rs::JellyfinClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JellyfinClient::new("http://example.com").await?;
    Ok(())
}
```

With Username and Password Authentication

```rust
use jellyfin_rs::JellyfinClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JellyfinClient::new_auth_name("http://example.com", "user", "password").await?;
    Ok(())
}
```

## API Reference

[comment]: <> (TODO)

## Error Handling

`jellyfin-rs` employs a custom error handling approach to gracefully manage various types of errors that may occur during API interaction. The library defines JellyfinError, an enum that encapsulates the different error scenarios you might encounter:

`NetworkError`: Occurs during network communication failures. It wraps reqwest::Error, which includes timeout issues, DNS failures, etc.
`UrlParseError`: Triggered when there's an issue parsing the Jellyfin server URL. It wraps url::ParseError.
`AuthNotFound`: Indicates that authentication information is missing or invalid. This is used when authentication with the server fails.
`HttpRequestError`: Represents errors related to HTTP requests, including but not limited to 4xx and 5xx HTTP response statuses. It includes additional context like the HTTP status code and a message describing the error.

All functions that interact with the Jellyfin server return a `Result<T, JellyfinError>`, allowing for comprehensive error handling in your application. Here's an example of handling different types of `JellyfinError`:

```rust
match jellyfin_client.some_operation().await {
    Ok(result) => {
        // Handle success
    },
    Err(e) => match e {
        JellyfinError::NetworkError(_) => {
            // Handle network error
        },
        JellyfinError::UrlParseError(_) => {
            // Handle URL parse error
        },
        JellyfinError::AuthNotFound => {
            // Handle authentication error
        },
        JellyfinError::HttpRequestError { status, message } => {
            // Handle HTTP request error, possibly log or display the status and message
        },
    }
}
```

## Contributing

Contributions to `jellyfin-rs` are welcome! Whether it's adding new features, fixing bugs, or improving documentation, your help is appreciated. Please submit pull requests or open issues on the project's GitHub page.

## License

`jellyfin-rs` is released under the MIT License. See the LICENSE file in the project repository for more details.

## Acknowledgments

This project, `jellyfin-rs`, is inspired by and based on the initial work done by [https://github.com/sargon64/jellyfin-rs](sargon64) on the Jellyfin Rust client. While that project has been on hiatus, it laid the foundation for what we've built here. We are grateful for sargon64's contributions to the community and their work that ignited this project. Our continuation of `jellyfin-rs` aims to expand on that foundation, addressing new API changes and furthering the Rust ecosystem's capabilities for interacting with Jellyfin.

