use crate::auth::SessionCheck;

use sword::prelude::*;
use sword::socketio::*;

#[controller(kind = Controller::SocketIo, namespace = "/attempts")]
#[interceptor(SessionCheck)]
pub struct AttemptsSocketIoController {}

impl AttemptsSocketIoController {
    #[on("connection")]
    async fn on_connect(&self, ctx: SocketContext) {
        tracing::info!("New socket /attempts connection: {}", ctx.id());
        ctx.socket.emit("attempts:connected", "OK").ok();
    }

    #[on("disconnection")]
    async fn on_disconnect(&self, ctx: SocketContext) {
        tracing::info!("Socket /attempts disconnected: {}", ctx.id());
    }
}
