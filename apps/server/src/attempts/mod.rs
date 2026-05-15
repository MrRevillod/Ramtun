mod controllers {
    pub mod socketio;
    pub mod web;
}

mod dtos;
mod entity;
mod errors;
mod repository;
mod services;
mod views;

use sword::prelude::*;

use controllers::socketio::AttemptsSocketIoController;
use controllers::web::AttemptsController;

pub use dtos::*;
pub use entity::*;
pub use errors::AttemptError;
pub use repository::AttemptRepository;
pub use services::*;
pub use views::*;

pub struct AttemptsModule;

impl Module for AttemptsModule {
    fn register_components(components: &ComponentRegistry) {
        components.register::<AttemptRepository>();
        components.register::<AttemptsService>();
        components.register::<QuestionService>();
        components.register::<GradingService>();
        components.register::<AnswerService>();
    }

    fn register_controllers(controllers: &ControllerRegistry) {
        controllers.register::<AttemptsController>();
        controllers.register::<AttemptsSocketIoController>();
    }
}
