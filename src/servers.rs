use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, ACCEPT, USER_AGENT};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct ServerInfo {
    pub id: u32,
    pub identifier: String,
    pub uuid: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct ServerObject {
    attributes: ServerInfo,
}

#[derive(Deserialize, Debug)]
pub struct WebResponse {
    data: Vec<ServerObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentState {
    pub current_state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub attributes: CurrentState
}

/// Get a list of all servers.
pub async fn get_servers() -> Result<Vec<ServerInfo>, Error> {
    let _server_uri: String = format!("{}/application/servers?per_page=500", std::env::var("SERVER_URI").unwrap());
    let _server_key: String = format!("Bearer {}", std::env::var("SERVER_KEY").unwrap());

    let client: Client = Client::new();
    let result = client.get(_server_uri)
        .headers(set_headers(_server_key))
        .send().await?;

    let response = result.json::<WebResponse>().await.unwrap();
    Ok(response.data.into_iter().map(|d| d.attributes).collect())
}

/// Get the state of a specified server.
///
/// # Arguments
///
/// * `identifier` - `&String` - The identifier of the server you want to check.
pub async fn get_server_state(identifier: &String) -> Result<String, Error> {
    let _server_uri: String = format!("{}/client/servers/{}/resources", std::env::var("SERVER_URI").unwrap(), identifier);
    let _client_key: String = format!("Bearer {}", std::env::var("CLIENT_KEY").unwrap());

    let client: Client = Client::new();

    let result = client.get(_server_uri)
        .headers(set_headers(_client_key))
        .send().await?;

    let response = result.json::<Attributes>().await.unwrap();
    Ok(response.attributes.current_state)
}

/// Get the state of a specified server.
///
/// # Arguments
///
/// * `identifier` - `&String` - The identifier of the server you want to check.
/// * `command` - `&str` - The command you want to send to the server.
pub async fn send_command(identifier: &String, command: &str) -> Result<String, Error> {
    let _server_uri: String = format!("{}/client/servers/{}/command", std::env::var("SERVER_URI").unwrap(), identifier);
    let _client_key: String = format!("Bearer {}", std::env::var("CLIENT_KEY").unwrap());

    let client: Client = Client::new();

    let result = client.post(_server_uri)
        .headers(set_headers(_client_key))
        .json(&json!({"command": command}))
        .send().await?;

    let results = result.text().await.unwrap();
    Ok(results)
}

/// Set all the headers for an HTTP request.
///
/// # Arguments
///
/// * `key` - `String` - The API token from Pterodactyl.
fn set_headers(key: String) -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(AUTHORIZATION, key.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(USER_AGENT, "guardian/0.1.0".parse().unwrap());
    return headers;
}
