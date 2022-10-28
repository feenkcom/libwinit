use crate::WinitUserEvent;
use std::error::Error;
use std::sync::PoisonError;
use thiserror::Error;
use value_box::BoxerError;
use winit::event_loop::EventLoopClosed;
use winit::window::WindowId;

#[derive(Error, Debug)]
pub enum WinitError {
    #[error("Winit error")]
    Winit(#[from] winit::error::OsError),
    #[error("Failed to lock the mutex")]
    PoisonError,
    #[error("Event loop is not running")]
    EventLoopNotRunning,
    #[error("Window with id {0:?} not found")]
    WindowNotFound(WindowId),
    #[error("Event loop closed")]
    EventLoopClosed(#[from] EventLoopClosed<WinitUserEvent>),
    #[error("Boxer error")]
    BoxerError(#[from] BoxerError),
}

impl WinitError {
    pub fn boxed(self) -> Box<dyn Error> {
        Box::new(self)
    }
}

impl<T> From<WinitError> for std::result::Result<T, WinitError> {
    fn from(error: WinitError) -> Self {
        Err(error)
    }
}

impl<T> From<PoisonError<T>> for WinitError {
    fn from(_err: PoisonError<T>) -> Self {
        Self::PoisonError
    }
}

impl From<WinitError> for BoxerError {
    fn from(error: WinitError) -> Self {
        BoxerError::AnyError(error.boxed())
    }
}

pub type Result<T> = core::result::Result<T, WinitError>;
