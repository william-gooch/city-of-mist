#![feature(type_alias_impl_trait)]

mod routes;
mod service;

use common::sea_orm::Database;
use dotenv::dotenv;
use routes::RoutesModule;
use service::database::DbImpl;
use service::ServiceModule;
use shaku::HasComponent;
use socket::*;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let service_module = Arc::new(
        ServiceModule::builder()
            .with_component_parameters::<DbImpl>(service::database::DbImplParameters {
                db: Arc::new(
                    Database::connect(env::var("DATABASE_URL").unwrap())
                        .await
                        .unwrap(),
                ),
            })
            .build(),
    );

    let routes_module = RoutesModule::builder(service_module).build();
    let root_routes: Arc<dyn routes::RootRoutes> = routes_module.resolve();

    warp::serve(root_routes.get_filter())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
