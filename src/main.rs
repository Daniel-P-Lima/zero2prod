use sqlx::PgPool;
use std::{env, net::TcpListener, ops::Sub};
use tracing::{
    Subscriber,
    subscriber::{self, set_global_default},
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", &configuration.application_port);
    let listener = TcpListener::bind(&address)
        .unwrap_or_else(|_| panic!("Failed to bound port {}", &configuration.application_port));
    run(listener, connection_pool)?.await
}
