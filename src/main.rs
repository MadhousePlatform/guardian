mod servers;
mod tolerance;

use std::thread;
use log::{info};
use dotenv::dotenv;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    info!("Init dotenv");
    dotenv().ok();

    info!("Init Sentry");
    let _guard = sentry::init((std::env::var("SENTRY_DSN"), sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    info!("Starting Guardian.");

    let map_thread = thread::spawn({
        /* Things are getting out of hand in this function so
         * it'll need to split off because this function should
         * only run once every couple of hours to ensure the
         * server list is up to date.
         */
        servers::map_servers.await;
        sleep(Duration::from_secs(120)).await;
    });

    /* Splitting this out into its own function because it will
     * likely be a long block of code and I don't want it cluttering
     * up my main function.
     */
    let ping_thread = thread::spawn(do_pinging);
}

async fn do_pinging() {
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
