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

pub async fn get_servers() -> Result<Vec<ServerInfo>, Error> {
    let _server_uri: String = format!("{}/application/servers?per_page=500", std::env::var("SERVER_URI").unwrap());

    let _server_key: String = format!("Bearer {}", std::env::var("SERVER_KEY").unwrap());

    let client: Client = Client::new();
    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(AUTHORIZATION, _server_key.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(USER_AGENT, "guardian/0.1.0".parse().unwrap());

    let result = client.get(_server_uri)
        .headers(headers)
        .send().await?;

    let response = result.json::<WebResponse>().await.unwrap();
    Ok(response.data.into_iter().map(|d| d.attributes).collect())
}

pub async fn get_server_state(id: &String) -> Result<String, Error> {
    let _server_uri: String = format!("{}/client/servers/{}/resources", std::env::var("SERVER_URI").unwrap(), id);

    let _server_key: String = format!("Bearer {}", std::env::var("CLIENT_KEY").unwrap());

    let client: Client = Client::new();
    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(AUTHORIZATION, _server_key.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(USER_AGENT, "guardian/0.1.0".parse().unwrap());

    let result = client.get(_server_uri)
        .headers(headers)
        .send().await?;

    let response = result.json::<Attributes>().await.unwrap();
    Ok(response.attributes.current_state)
}

pub async fn send_command(server: &String, command: &str) -> Result<String, Error> {
    let _server_uri: String = format!("{}/client/servers/{}/command", std::env::var("SERVER_URI").unwrap(), server);
    let _client_key: String = format!("Bearer {}", std::env::var("CLIENT_KEY").unwrap());

    let client: Client = Client::new();
    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(AUTHORIZATION, _client_key.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(USER_AGENT, "guardian/0.1.0".parse().unwrap());

    let result = client.post(_server_uri)
        .headers(headers)
        .json(
            &json!({"command": command})
        )
        .send().await?;

    let results = result.text().await.unwrap();
    Ok(results)
}
