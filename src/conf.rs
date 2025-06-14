//! The `conf` module contains functions for loading and saving game
//! configurations.
//!
//! A [`Conf`](struct.Conf.html) struct is used to create a config file
//! which specifies hardware setup stuff, mostly video display settings.
//!
//! By default a ggez game will search its resource paths for a `/conf.toml`
//! file and load values from it when the [`Context`](../struct.Context.html) is created.  This file
//! must be complete (ie you cannot just fill in some fields and have the
//! rest be default) and provides a nice way to specify settings that
//! can be tweaked such as window resolution, multisampling options, etc.
//! If no file is found, it will create a `Conf` object from the settings
//! passed to the [`ContextBuilder`](../struct.ContextBuilder.html).

use std::convert::TryFrom;

use winit::dpi::PhysicalSize;

use crate::error::{GameError, GameResult};

/// Possible fullscreen modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FullscreenType {
    /// Windowed mode.
    Windowed,
    /// True fullscreen, which used to be preferred 'cause it can have
    /// small performance benefits over windowed fullscreen.
    ///
    /// Also it allows us to set different resolutions.
    True,
    /// Windowed fullscreen, generally preferred over real fullscreen
    /// these days 'cause it plays nicer with multiple monitors.
    Desktop,
}

/// A builder structure containing window settings
/// that can be set at runtime and changed with [`graphics::set_mode()`](../graphics/fn.set_mode.html).
///
/// Defaults:
///
/// ```rust
/// # use ggez::conf::*;
/// # fn main() { assert_eq!(
/// WindowMode {
///     width: 800.0,
///     height: 600.0,
///     maximized: false,
///     fullscreen_type: FullscreenType::Windowed,
///     borderless: false,
///     min_width: 1.0,
///     max_width: 0.0,
///     min_height: 1.0,
///     max_height: 0.0,
///     resizable: false,
///     visible: true,
///     transparent: false,
///     resize_on_scale_factor_change: false,
///     logical_size: None,
/// }
/// # , WindowMode::default());}
/// ```
#[derive(Debug, Copy, Clone, smart_default::SmartDefault, PartialEq)]
pub struct WindowMode {
    /// Window width in physical pixels
    #[default = 800.0]
    pub width: f32,
    /// Window height in physical pixels
    #[default = 600.0]
    pub height: f32,
    /// Whether or not to maximize the window
    #[default = false]
    pub maximized: bool,
    /// Fullscreen type
    #[default(FullscreenType::Windowed)]
    pub fullscreen_type: FullscreenType,
    /// Whether or not to show window decorations
    #[default = false]
    pub borderless: bool,
    /// Whether or not the window should be transparent
    #[default = false]
    pub transparent: bool,
    /// Minimum width for resizable windows; 1 is the technical minimum,
    /// as wgpu will panic on a width of 0.
    #[default = 1.0]
    pub min_width: f32,
    /// Minimum height for resizable windows; 1 is the technical minimum,
    /// as wgpu will panic on a height of 0.
    #[default = 1.0]
    pub min_height: f32,
    /// Maximum width for resizable windows; 0 means no limit
    #[default = 0.0]
    pub max_width: f32,
    /// Maximum height for resizable windows; 0 means no limit
    #[default = 0.0]
    pub max_height: f32,
    /// Whether or not the window is resizable
    #[default = false]
    pub resizable: bool,
    /// Whether this window should displayed (true) or hidden (false)
    #[default = true]
    pub visible: bool,
    /// Whether this window should change its size in physical pixels
    /// when its hidpi factor changes, i.e. when [`WindowEvent::ScaleFactorChanged`](https://docs.rs/winit/0.25.0/winit/event/enum.WindowEvent.html#variant.ScaleFactorChanged)
    /// is fired.
    ///
    /// You usually want this to be false, since the window suddenly changing size may break your game.
    /// Setting this to true may be desirable if you plan for it and want your window to behave like
    /// windows of other programs when being dragged from one screen to another, for example.
    ///
    /// For more context on this take a look at [this conversation](https://github.com/ggez/ggez/pull/949#issuecomment-854731226).
    #[default = false]
    pub resize_on_scale_factor_change: bool,
    // logical_size is serialized as a table, so it must be at the end of the struct for toml
    /// Window height/width but allows LogicalSize for high DPI systems. If Some will be used instead of width/height.
    #[default(None)]
    pub logical_size: Option<winit::dpi::LogicalSize<f32>>,
}

impl WindowMode {
    /// Set default window size, or screen resolution in true fullscreen mode.
    #[must_use]
    pub fn dimensions(mut self, width: f32, height: f32) -> Self {
        if width >= 1.0 {
            self.width = width;
        }
        if height >= 1.0 {
            self.height = height;
        }
        self
    }

    /// Set whether the window should be maximized.
    #[must_use]
    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    /// Set the fullscreen type.
    #[must_use]
    pub fn fullscreen_type(mut self, fullscreen_type: FullscreenType) -> Self {
        self.fullscreen_type = fullscreen_type;
        self
    }

    /// Set whether a window should be borderless in windowed mode.
    #[must_use]
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Set whether a window should be transparent.
    #[must_use]
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// Set minimum window dimensions for windowed mode.
    /// Minimum dimensions will always be >= 1.
    #[must_use]
    pub fn min_dimensions(mut self, width: f32, height: f32) -> Self {
        if width >= 1.0 {
            self.min_width = width;
        }
        if height >= 1.0 {
            self.min_height = height;
        }
        self
    }

    /// Set maximum window dimensions for windowed mode.
    #[must_use]
    pub fn max_dimensions(mut self, width: f32, height: f32) -> Self {
        self.max_width = width;
        self.max_height = height;
        self
    }

    /// Set resizable.
    #[must_use]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Set visibility
    #[must_use]
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set whether to resize when the hidpi factor changes
    #[must_use]
    pub fn resize_on_scale_factor_change(mut self, resize_on_scale_factor_change: bool) -> Self {
        self.resize_on_scale_factor_change = resize_on_scale_factor_change;
        self
    }

    // Use logical_size if set, else convert width/height to PhysicalSize
    pub(crate) fn actual_size(&self) -> GameResult<winit::dpi::Size> {
        let actual_size: winit::dpi::Size = if let Some(logical_size) = self.logical_size {
            logical_size.into()
        } else {
            winit::dpi::PhysicalSize::<f64>::from((self.width, self.height)).into()
        };

        let physical_size: PhysicalSize<f64> = actual_size.to_physical(1.0);
        if physical_size.width >= 1.0 && physical_size.height >= 1.0 {
            Ok(actual_size)
        } else {
            Err(GameError::WindowError(format!(
                "window width and height need to be at least 1; actual values: {}, {}",
                physical_size.width, physical_size.height
            )))
        }
    }
}

/// A builder structure containing window settings
/// that must be set at init time and cannot be changed afterwards.
///
/// Defaults:
///
/// ```rust
/// # use ggez::conf::*;
/// # fn main() { assert_eq!(
/// WindowSetup {
///     title: "An easy, good game".to_owned(),
///     samples: NumSamples::One,
///     vsync: true,
///     icon: "".to_owned(),
///     srgb: true,
/// }
/// # , WindowSetup::default()); }
/// ```
#[derive(Debug, Clone, smart_default::SmartDefault, PartialEq, Eq)]
pub struct WindowSetup {
    /// The window title.
    #[default(String::from("An easy, good game"))]
    pub title: String,
    /// The window's general name.
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub general_name: Option<String>,
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    /// The window's instance name.
    pub instance_name: Option<String>,
    /// Number of samples to use for multisample anti-aliasing.
    #[default(NumSamples::One)]
    pub samples: NumSamples,
    /// Whether or not to enable vsync.
    #[default = true]
    pub vsync: bool,
    /// A file path to the window's icon.
    /// It takes a path rooted in the `resources` directory (see the [`filesystem`](../filesystem/index.html)
    /// module for details), and an empty string results in a blank/default icon.
    #[default(String::new())]
    pub icon: String,
    /// Whether or not to enable sRGB (gamma corrected color)
    /// handling on the display.
    #[default = true]
    pub srgb: bool,
}

impl WindowSetup {
    /// Set window title.
    #[must_use]
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    /// Set window's general and instance name.
    #[must_use]
    pub fn name(mut self, general: &str, instance: &str) -> Self {
        self.general_name = Some(general.to_owned());
        self.instance_name = Some(instance.to_owned());
        self
    }

    /// Set number of samples to use for multisample anti-aliasing.
    #[must_use]
    pub fn samples(mut self, samples: NumSamples) -> Self {
        self.samples = samples;
        self
    }

    /// Set whether vsync is enabled.
    #[must_use]
    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    /// Set the window's icon.
    #[must_use]
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = icon.to_owned();
        self
    }

    /// Set `sRGB` color mode.
    #[must_use]
    pub fn srgb(mut self, active: bool) -> Self {
        self.srgb = active;
        self
    }
}

/// Possible graphics backends.
/// The default is `Primary`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, smart_default::SmartDefault)]
pub enum Backend {
    /// Includes [`Backend::OnlyPrimary`] and also secondary APIs consisting of OpenGL and DX11.
    ///
    /// These APIs may have issues and may be deprecated by some platforms.
    #[default]
    All,
    /// Primary APIs consisting of Vulkan, Metal and DX12.
    ///
    /// These APIs have first-class support from WGPU and from the platforms that support them.
    OnlyPrimary,
    /// Use the Khronos Vulkan API.
    Vulkan,
    /// Use the Apple Metal API.
    Metal,
    /// Use the Microsoft DirectX 12 API.
    Dx12,
    /// Use the Microsoft DirectX 11 API. This is not a recommended backend.
    Dx11,
    /// Use the Khronos OpenGL API. This is not a recommended backend.
    Gl,
    /// Use the WebGPU API. Targets the web.
    BrowserWebGpu,
}

/// The possible number of samples for multisample anti-aliasing.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NumSamples {
    /// One sample
    One = 1,
    /* uncomment when WGPU supports more sample counts
    /// Two samples
    Two = 2,
    */
    /// Four samples
    Four = 4,
    /* uncomment when WGPU supports more sample counts
    /// Eight samples
    Eight = 8,
    /// Sixteen samples
    Sixteen = 16,
    */
}

impl TryFrom<u8> for NumSamples {
    type Error = GameError;
    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(NumSamples::One),
            //2 => Ok(NumSamples::Two),
            4 => Ok(NumSamples::Four),
            //8 => Ok(NumSamples::Eight),
            //16 => Ok(NumSamples::Sixteen),
            _ => Err(GameError::ConfigError(String::from(
                "Invalid number of samples",
            ))),
        }
    }
}

impl From<NumSamples> for u8 {
    fn from(ns: NumSamples) -> u8 {
        ns as u8
    }
}

/// A structure containing configuration data
/// for the game engine.
///
/// Defaults:
///
/// ```rust
/// # use ggez::conf::*;
/// # fn main() { assert_eq!(
/// Conf {
///     window_mode: WindowMode::default(),
///     window_setup: WindowSetup::default(),
///     backend: Backend::default(),
/// }
/// # , Conf::default()); }
/// ```
#[derive(Debug, PartialEq, smart_default::SmartDefault, Clone)]
pub struct Conf {
    /// Window setting information that can be set at runtime
    pub window_mode: WindowMode,
    /// Window setting information that must be set at init-time
    pub window_setup: WindowSetup,
    /// Graphics backend configuration
    pub backend: Backend,
}

impl Conf {
    /// Same as `Conf::default()`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the window mode
    #[must_use]
    pub fn window_mode(mut self, window_mode: WindowMode) -> Self {
        self.window_mode = window_mode;
        self
    }

    /// Sets the backend
    #[must_use]
    pub fn backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }
}
