use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, ACCEPT, USER_AGENT};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::Instant;

use crate::{tolerance};

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
    pub attributes: CurrentState,
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

/// Get a list of all servers.
pub async fn map_servers() -> HashMap<String, HashMap<String, String>> {
    let servers: Result<Vec<ServerInfo>, Error> = get_servers().await;
    match servers {
        Ok(servers) => {
            /* This is cheap, nasty and vulgar but we're using a
             * HashMap inside a HashMap to emulate a PHP array
             * because I don't know how else to store this data.
             * - Sketch, 28/12/23
             */
            let mut server_map = HashMap::new();

            /* Index the HashMap inside the HashMap from 0 */
            let mut i = 0;
            for server in servers {
                let p = ping_server(&server).await;
                // We don't want to check stopped servers.
                if p != "stopped" {
                    /* Build the inner HashMap */
                    let mut inner = HashMap::new();
                    inner.insert("server".to_string(), server.identifier.to_string());
                    inner.insert("name".to_string(), server.name.to_string());
                    inner.insert("slow".to_string(), 4.to_string());
                    /* Insert inner HashMap to outer HashMap */
                    server_map.insert(i.to_string(), inner);
                }
                /* Increment the index */
                i += 1;
            }

            println!("{:?}", server_map);
            return server_map;
        }
        Err(e) => {
            panic!("An error occurred: {:?}", e)
        }
    }
}

/// Ping the specified server and check if the response
/// time is within tolerance.
///
/// # Arguments
///
/// * `server` - `&ServerInfo` - The identifier of the server you want to check.
///
/// # Returns
///
/// `&str` - "fast", "slow", "stopped".
pub async fn ping_server(server: &ServerInfo) -> &str {
    let id: &String = &server.identifier;
    let response: Result<String, Error> = get_server_state(id).await;

    if response.unwrap().trim() == "running" {
        let duration: Instant = Instant::now();

        send_list_command(id).await;
        let elapsed: u128 = duration.elapsed().as_millis();
        let tol: &str = tolerance::within_tolerance(elapsed);
        println!("server: {:?}, time: {:?}ms, within tol (1200ms): {:?}", &server.name, elapsed, &tol);
        return tol;
    }

    return "stopped";
}

/*async fn send_tps_request(server: String) {
    servers::send_command(server.parse().unwrap(), "forge tps").await.expect("()");
}
*/

/// Send the LIST command to the minecraft server.
///
/// # Arguments
///
/// * `identifier` - `&String` - The identifier of the server you want to check.
pub async fn send_list_command(identifier: &String) {
    send_command(identifier, "list").await.expect("()");
}
