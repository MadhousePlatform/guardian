mod servers;

use log::{info};
use dotenv::dotenv;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    info!("Init dotenv");

    dotenv().ok();
    info!("Starting Guardian.");

    loop {
        map_servers().await;


        sleep(Duration::from_secs(5)).await;
    }
}

async fn map_servers() {
    let servers = servers::get_servers().await;
    match servers {
        Ok(servers) => {
            for server in servers {
                let id = &server.identifier;
                let response = servers::get_server_state(id).await;
                if response.unwrap().trim() == "running" {
                    let r = send_list_command(id).await;
                }
            }
        }
        Err(e) => {
            println!("ERROR: Error getting server list: {}", e)
        }
    }
}

/*async fn send_tps_request(server: String) {
    servers::send_command(server.parse().unwrap(), "forge tps").await.expect("()");
}
*/
async fn send_list_command(server: &String) {
    servers::send_command(server, "list").await.expect("()");
}
