use std::sync::{Arc, Mutex};

use core::default::Default;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::WindowId;

#[derive(Debug, Clone)]
pub struct WindowRef {
    id: WindowId,
    data: Arc<Mutex<WindowData>>,
}

impl WindowRef {
    pub fn new(id: &WindowId) -> Self {
        Self {
            id: id.clone(),
            data: Default::default(),
        }
    }

    /// Return an id of the window. Can be called from any thread
    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn scale_factor(&self) -> crate::Result<f64> {
        self.data
            .lock()
            .map_err(|error| error.into())
            .map(|lock| lock.scale_factor)
    }

    pub fn outer_position(&self) -> crate::Result<PhysicalPosition<i32>> {
        self.data
            .lock()
            .map_err(|error| error.into())
            .map(|lock| lock.outer_position.clone())
    }

    pub fn inner_size(&self) -> crate::Result<PhysicalSize<u32>> {
        self.data
            .lock()
            .map_err(|error| error.into())
            .map(|lock| lock.inner_size.clone())
    }

    pub fn set_inner_size(&self, size: PhysicalSize<u32>) -> crate::Result<()> {
        self.data
            .lock()
            .map_err(|error| error.into())
            .map(|mut lock| lock.inner_size = size)
    }

    pub fn set_scale_factor(&self, scale_factor: f64) -> crate::Result<()> {
        self.data
            .lock()
            .map_err(|error| error.into())
            .map(|mut lock| lock.scale_factor = scale_factor)
    }

    pub fn set_outer_position(&self, position: PhysicalPosition<i32>) -> crate::Result<()> {
        self.data
            .lock()
            .map_err(|error| error.into())
            .map(|mut lock| lock.outer_position = position)
    }
}

#[derive(Debug, Clone, Default)]
struct WindowData {
    outer_position: PhysicalPosition<i32>,
    inner_size: PhysicalSize<u32>,
    scale_factor: f64,
}
