mod servers;

use log::{info};
use dotenv::dotenv;
use tokio::time::{sleep, Duration, Instant};

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
                let now = Instant::now();
                let r = send_list_command(id.parse().unwrap()).await;
                let elapsed_time = now.elapsed();
                println!("List took {} milliseconds.", elapsed_time.as_millis());
                println!("{:?}", r);
                println!("INFO: {} - {}", id, server.name.as_str())
            }
        }
        Err(e) => {
            println!("ERROR: Error getting server list: {}", e)
        }
    }
}

/*async fn send_tps_request(server: String) {
    servers::send_command(server, "forge tps").await;
}
*/
async fn send_list_command(server: String) {
    servers::send_command(server.parse().unwrap(), "list").await.expect("TODO: panic message");
}
