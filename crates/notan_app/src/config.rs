use crate::backend::Backend;
use crate::builder::{AppBuilder, BuildConfig};

/// Builder configuration for the window options
#[derive(Clone)]
pub struct WindowConfig {
    /// Window's title
    /// `Web: no-op`
    pub title: String,

    /// Window's width
    pub width: i32,

    /// Window's height
    pub height: i32,

    /// Start window in fullscreen mode
    /// `Web: no-op`
    pub fullscreen: bool,

    /// Minimum resizable window's size
    pub min_size: Option<(i32, i32)>,

    /// Maximum resizable window's size
    pub max_size: Option<(i32, i32)>,

    /// Start the window maximized
    /// `Web: no-op`
    pub maximized: bool,

    /// Allow to resize the window
    /// `Web: no-op`
    pub resizable: bool,

    /// Enable V-Sync
    /// `Web: no-op`
    pub vsync: bool,

    /// Antialias nultisamples level
    /// `Web: WebGL will use this as antialias = false if the value is 0 or true otherwise`
    pub multisampling: u16,

    /// **Only Web:** By default a canvas will have the size set multiplied by the device_pixel_ratio
    /// This can be disabled by setting this to `false`. This could be useful for mobile browsers.
    pub canvas_auto_resolution: bool,

    /// Inner loop will run only after an input event
    pub lazy_loop: bool,

    /// Background as transparent
    pub transparent: bool,

    /// Enable decorations
    /// `Web: Does nothing`
    pub decorations: bool,

    /// Hide the windows
    pub visible: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Notan App"),
            width: 800,
            height: 600,
            fullscreen: false,
            min_size: None,
            max_size: None,
            maximized: false,
            resizable: false,
            vsync: false,
            multisampling: 0,
            canvas_auto_resolution: true,
            lazy_loop: false,
            transparent: false,
            decorations: true,
            visible: true,
        }
    }
}

impl WindowConfig {
    /// Create a new instance using default values
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the window's title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Inner loop will run only after an input event
    pub fn lazy_loop(mut self, lazy: bool) -> Self {
        self.lazy_loop = lazy;
        self
    }

    /// Sets the window's width and height
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Enable fullscreen mode
    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    /// Sets the window's minimum size
    pub fn min_size(mut self, width: i32, height: i32) -> Self {
        self.min_size = Some((width, height));
        self
    }

    /// Sets the window's maximum size
    pub fn max_size(mut self, width: i32, height: i32) -> Self {
        self.max_size = Some((width, height));
        self
    }

    /// Starts the window maximized
    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    /// Allow the window to be resizable
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Enable vsync
    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    /// Enabled multisampling aliasing (opengl)
    pub fn multisampling(mut self, samples: u16) -> Self {
        self.multisampling = samples;
        self
    }

    /// Enable or disable that the size of the canvas will automatically use the device pixel ratio
    pub fn canvas_auto_resolution(mut self, enabled: bool) -> Self {
        self.canvas_auto_resolution = enabled;
        self
    }

    /// Set the background as transparent
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// Enable or disable decorations
    pub fn decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    /// Hide or show the window
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl<S, B> BuildConfig<S, B> for WindowConfig
where
    B: Backend,
{
    fn apply(&self, mut builder: AppBuilder<S, B>) -> AppBuilder<S, B> {
        builder.window = self.clone();
        builder
    }
}
