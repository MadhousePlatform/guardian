mod servers;
mod tolerance;

use std::collections::HashMap;
use log::{info};
use dotenv::dotenv;
use reqwest::Error;
use tokio::time::{sleep, Duration, Instant};
use crate::servers::ServerInfo;

#[tokio::main]
async fn main() {
    info!("Init dotenv");

    dotenv().ok();
    info!("Starting Guardian.");

    loop {
        /* Things are getting out of hand in this function so
         * it'll need to split off because this function should
         * only run once every couple of hours to ensure the
         * server list is up to date.
         */
        map_servers().await;

        /* Then we want to use the HashMap of servers to ping
         * the Pterodactyl API to send the LIST and TPS commands
         * (where supported) to decide if the server is running
         * slowly and if it is, increment the SLOW counter by 1.
         * If the server is within tolerance on the next check after
         * SLOW was incremented, we'll reset it.
         * This should run every 10 - 15 seconds.
         */

        /* This is where we will want to issue restart commands
         * if a server is SLOW == 3 times in a row.
         */

        sleep(Duration::from_secs(5)).await;
    }
}

/// Get a list of all servers.
async fn map_servers() {
    let servers: Result<Vec<ServerInfo>, Error> = servers::get_servers().await;
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
                    inner.insert("server", server.identifier);
                    inner.insert("name", server.name);
                    inner.insert("slow", 4.to_string());
                    /* Insert inner HashMap to outer HashMap */
                    server_map.insert(i.to_string(), inner);
                }
                /* Increment the index */
                i += 1;
            }

            println!("{:?}", server_map);
        }
        Err(e) => {
            println!("ERROR: Error getting server list: {}", e)
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
async fn ping_server(server: &ServerInfo) -> &str {
    let id: &String = &server.identifier;
    let response: Result<String, Error> = servers::get_server_state(id).await;

    if response.unwrap().trim() == "running" {
        let duration: Instant = Instant::now();

        send_list_command(id).await;
        let elapsed: u128 = duration.elapsed().as_millis();
        let tol: &str = tolerance::within_tolerance(elapsed);
        println!("server: {:?}, time: {:?}ms, within tol (1200ms): {:?}", &server.name, elapsed, &tol);
        return tol
    }

    return "stopped"
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
async fn send_list_command(identifier: &String) {
    servers::send_command(identifier, "list").await.expect("()");
}
