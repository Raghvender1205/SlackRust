use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};

/// Define a WebSocket actor that will handle incoming messages.
struct WebSocketChatSession;

impl Actor for WebSocketChatSession {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketChatSession {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

/// Entry point for WebSocket connections
async fn chat_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebSocketChatSession {}, &req, stream)
}

/// Start HTTP Server
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ws/", web::get().to(chat_route))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}