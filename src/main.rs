use serde::Serialize;
use serde_derive::{Deserialize, Serialize};
use thruster::{App, async_middleware, BasicContext as Ctx, middleware_fn, MiddlewareNext, MiddlewareResult, Request};
use thruster::{Server, ThrusterServer};
use thruster::errors::ThrusterError;


#[derive(Serialize, Default)]
struct WeatherResponse {
    temperature: f32,
}


#[middleware_fn]
async fn page404(mut context: Ctx, _: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    context.status(404);
    context.body("Not Found (404). Ok?");
    Ok(context)
}


#[middleware_fn]
async fn get_temperature(mut context: Ctx, _: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let answer = WeatherResponse::default();
    match serde_json::to_string(&answer) {
        Ok(body) => {
            context.body(&body);
            context.headers.insert("Content-Type".to_string(), "application/json".to_string());
            Ok(context)
        }
        Err(_) => {
            Err(ThrusterError {
                context,
                message: "Unprocessed server err".to_string(),
                status: 500,
                cause: None,
            })
        }
    }
}


#[tokio::main]
async fn main() {
    let mut app = App::<Request, Ctx, ()>::new_basic();

    app.set404(async_middleware!(Ctx, [page404]));
    app.get("/temperature", async_middleware!(Ctx, [get_temperature]));

    let server = Server::new(app);
    server.build("127.0.0.1", 8080).await;
}
