use crate::auth::SessionCheck;

use sword::prelude::*;
use sword::socketio::*;

#[controller(kind = Controller::SocketIo, namespace = "/attempts")]
#[interceptor(SessionCheck)]
pub struct AttemptsSocketIoController;

impl AttemptsSocketIoController {
    #[on("connection")]
    async fn on_connect(&self, socket: SocketContext) {
        tracing::info!("New socket /attempts connection: {}", socket.id());
        socket.emit("attempts:connected", "OK").ok();
    }

    #[on("disconnection")]
    async fn on_disconnect(&self, socket: SocketContext) {
        tracing::info!("Socket /attempts disconnected: {}", socket.id());
    }
}
