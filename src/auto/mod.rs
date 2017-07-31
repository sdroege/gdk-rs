// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

mod app_launch_context;
pub use self::app_launch_context::AppLaunchContext;
pub use self::app_launch_context::AppLaunchContextExt;

mod cursor;
pub use self::cursor::Cursor;
pub use self::cursor::CursorExt;

mod device;
pub use self::device::Device;
pub use self::device::DeviceExt;

mod device_manager;
pub use self::device_manager::DeviceManager;
pub use self::device_manager::DeviceManagerExt;

#[cfg(feature = "v3_22")]
mod device_tool;
#[cfg(feature = "v3_22")]
pub use self::device_tool::DeviceTool;
#[cfg(feature = "v3_22")]
pub use self::device_tool::DeviceToolExt;

mod display;
pub use self::display::Display;
pub use self::display::DisplayExt;

mod display_manager;
pub use self::display_manager::DisplayManager;
pub use self::display_manager::DisplayManagerExt;

mod drag_context;
pub use self::drag_context::DragContext;
pub use self::drag_context::DragContextExt;

#[cfg(feature = "v3_22")]
mod drawing_context;
#[cfg(feature = "v3_22")]
pub use self::drawing_context::DrawingContext;
#[cfg(feature = "v3_22")]
pub use self::drawing_context::DrawingContextExt;

#[cfg(feature = "v3_8")]
mod frame_clock;
#[cfg(feature = "v3_8")]
pub use self::frame_clock::FrameClock;
#[cfg(feature = "v3_8")]
pub use self::frame_clock::FrameClockExt;

#[cfg(feature = "v3_16")]
mod g_l_context;
#[cfg(feature = "v3_16")]
pub use self::g_l_context::GLContext;
#[cfg(feature = "v3_16")]
pub use self::g_l_context::GLContextExt;

#[cfg(feature = "v3_22")]
mod monitor;
#[cfg(feature = "v3_22")]
pub use self::monitor::Monitor;
#[cfg(feature = "v3_22")]
pub use self::monitor::MonitorExt;

mod screen;
pub use self::screen::Screen;
pub use self::screen::ScreenExt;

#[cfg(feature = "v3_20")]
mod seat;
#[cfg(feature = "v3_20")]
pub use self::seat::Seat;
#[cfg(feature = "v3_20")]
pub use self::seat::SeatExt;

mod visual;
pub use self::visual::Visual;
pub use self::visual::VisualExt;

mod window;
pub use self::window::Window;
pub use self::window::WindowExt;

#[cfg(feature = "v3_8")]
mod frame_timings;
#[cfg(feature = "v3_8")]
pub use self::frame_timings::FrameTimings;

mod enums;
pub use self::enums::AxisUse;
pub use self::enums::ByteOrder;
pub use self::enums::CrossingMode;
pub use self::enums::CursorType;
#[cfg(feature = "v3_22")]
pub use self::enums::DeviceToolType;
pub use self::enums::DeviceType;
#[cfg(feature = "v3_20")]
pub use self::enums::DragCancelReason;
pub use self::enums::DragProtocol;
pub use self::enums::EventType;
#[cfg(feature = "v3_8")]
pub use self::enums::FullscreenMode;
#[cfg(feature = "v3_16")]
pub use self::enums::GLError;
pub use self::enums::GrabOwnership;
pub use self::enums::GrabStatus;
pub use self::enums::Gravity;
pub use self::enums::InputMode;
pub use self::enums::InputSource;
pub use self::enums::ModifierIntent;
pub use self::enums::NotifyType;
pub use self::enums::OwnerChange;
pub use self::enums::PropertyState;
pub use self::enums::ScrollDirection;
pub use self::enums::SettingAction;
#[cfg(feature = "v3_22")]
pub use self::enums::SubpixelLayout;
pub use self::enums::VisibilityState;
pub use self::enums::VisualType;
pub use self::enums::WindowEdge;
pub use self::enums::WindowType;
pub use self::enums::WindowTypeHint;
pub use self::enums::WindowWindowClass;

mod flags;
#[cfg(feature = "v3_22")]
pub use self::flags::AnchorHints;
#[cfg(feature = "v3_22")]
pub use self::flags::ANCHOR_FLIP_X;
#[cfg(feature = "v3_22")]
pub use self::flags::ANCHOR_FLIP_Y;
#[cfg(feature = "v3_22")]
pub use self::flags::ANCHOR_SLIDE_X;
#[cfg(feature = "v3_22")]
pub use self::flags::ANCHOR_SLIDE_Y;
#[cfg(feature = "v3_22")]
pub use self::flags::ANCHOR_RESIZE_X;
#[cfg(feature = "v3_22")]
pub use self::flags::ANCHOR_RESIZE_Y;
#[cfg(feature = "v3_22")]
pub use self::flags::ANCHOR_FLIP;
#[cfg(feature = "v3_22")]
pub use self::flags::ANCHOR_SLIDE;
#[cfg(feature = "v3_22")]
pub use self::flags::ANCHOR_RESIZE;
#[cfg(feature = "v3_22")]
pub use self::flags::AxisFlags;
#[cfg(feature = "v3_22")]
pub use self::flags::AXIS_FLAG_X;
#[cfg(feature = "v3_22")]
pub use self::flags::AXIS_FLAG_Y;
#[cfg(feature = "v3_22")]
pub use self::flags::AXIS_FLAG_PRESSURE;
#[cfg(feature = "v3_22")]
pub use self::flags::AXIS_FLAG_XTILT;
#[cfg(feature = "v3_22")]
pub use self::flags::AXIS_FLAG_YTILT;
#[cfg(feature = "v3_22")]
pub use self::flags::AXIS_FLAG_WHEEL;
#[cfg(feature = "v3_22")]
pub use self::flags::AXIS_FLAG_DISTANCE;
#[cfg(feature = "v3_22")]
pub use self::flags::AXIS_FLAG_ROTATION;
#[cfg(feature = "v3_22")]
pub use self::flags::AXIS_FLAG_SLIDER;
pub use self::flags::DragAction;
pub use self::flags::ACTION_DEFAULT;
pub use self::flags::ACTION_COPY;
pub use self::flags::ACTION_MOVE;
pub use self::flags::ACTION_LINK;
pub use self::flags::ACTION_PRIVATE;
pub use self::flags::ACTION_ASK;
pub use self::flags::EventMask;
pub use self::flags::EXPOSURE_MASK;
pub use self::flags::POINTER_MOTION_MASK;
pub use self::flags::POINTER_MOTION_HINT_MASK;
pub use self::flags::BUTTON_MOTION_MASK;
pub use self::flags::BUTTON1_MOTION_MASK;
pub use self::flags::BUTTON2_MOTION_MASK;
pub use self::flags::BUTTON3_MOTION_MASK;
pub use self::flags::BUTTON_PRESS_MASK;
pub use self::flags::BUTTON_RELEASE_MASK;
pub use self::flags::KEY_PRESS_MASK;
pub use self::flags::KEY_RELEASE_MASK;
pub use self::flags::ENTER_NOTIFY_MASK;
pub use self::flags::LEAVE_NOTIFY_MASK;
pub use self::flags::FOCUS_CHANGE_MASK;
pub use self::flags::STRUCTURE_MASK;
pub use self::flags::PROPERTY_CHANGE_MASK;
pub use self::flags::VISIBILITY_NOTIFY_MASK;
pub use self::flags::PROXIMITY_IN_MASK;
pub use self::flags::PROXIMITY_OUT_MASK;
pub use self::flags::SUBSTRUCTURE_MASK;
pub use self::flags::SCROLL_MASK;
pub use self::flags::TOUCH_MASK;
pub use self::flags::SMOOTH_SCROLL_MASK;
pub use self::flags::TOUCHPAD_GESTURE_MASK;
pub use self::flags::TABLET_PAD_MASK;
pub use self::flags::ALL_EVENTS_MASK;
#[cfg(feature = "v3_8")]
pub use self::flags::FrameClockPhase;
#[cfg(feature = "v3_8")]
pub use self::flags::FRAME_CLOCK_PHASE_NONE;
#[cfg(feature = "v3_8")]
pub use self::flags::FRAME_CLOCK_PHASE_FLUSH_EVENTS;
#[cfg(feature = "v3_8")]
pub use self::flags::FRAME_CLOCK_PHASE_BEFORE_PAINT;
#[cfg(feature = "v3_8")]
pub use self::flags::FRAME_CLOCK_PHASE_UPDATE;
#[cfg(feature = "v3_8")]
pub use self::flags::FRAME_CLOCK_PHASE_LAYOUT;
#[cfg(feature = "v3_8")]
pub use self::flags::FRAME_CLOCK_PHASE_PAINT;
#[cfg(feature = "v3_8")]
pub use self::flags::FRAME_CLOCK_PHASE_RESUME_EVENTS;
#[cfg(feature = "v3_8")]
pub use self::flags::FRAME_CLOCK_PHASE_AFTER_PAINT;
pub use self::flags::ModifierType;
pub use self::flags::SHIFT_MASK;
pub use self::flags::LOCK_MASK;
pub use self::flags::CONTROL_MASK;
pub use self::flags::MOD1_MASK;
pub use self::flags::MOD2_MASK;
pub use self::flags::MOD3_MASK;
pub use self::flags::MOD4_MASK;
pub use self::flags::MOD5_MASK;
pub use self::flags::BUTTON1_MASK;
pub use self::flags::BUTTON2_MASK;
pub use self::flags::BUTTON3_MASK;
pub use self::flags::BUTTON4_MASK;
pub use self::flags::BUTTON5_MASK;
pub use self::flags::MODIFIER_RESERVED_13_MASK;
pub use self::flags::MODIFIER_RESERVED_14_MASK;
pub use self::flags::MODIFIER_RESERVED_15_MASK;
pub use self::flags::MODIFIER_RESERVED_16_MASK;
pub use self::flags::MODIFIER_RESERVED_17_MASK;
pub use self::flags::MODIFIER_RESERVED_18_MASK;
pub use self::flags::MODIFIER_RESERVED_19_MASK;
pub use self::flags::MODIFIER_RESERVED_20_MASK;
pub use self::flags::MODIFIER_RESERVED_21_MASK;
pub use self::flags::MODIFIER_RESERVED_22_MASK;
pub use self::flags::MODIFIER_RESERVED_23_MASK;
pub use self::flags::MODIFIER_RESERVED_24_MASK;
pub use self::flags::MODIFIER_RESERVED_25_MASK;
pub use self::flags::SUPER_MASK;
pub use self::flags::HYPER_MASK;
pub use self::flags::META_MASK;
pub use self::flags::MODIFIER_RESERVED_29_MASK;
pub use self::flags::RELEASE_MASK;
pub use self::flags::MODIFIER_MASK;
#[cfg(feature = "v3_20")]
pub use self::flags::SeatCapabilities;
#[cfg(feature = "v3_20")]
pub use self::flags::SEAT_CAPABILITY_NONE;
#[cfg(feature = "v3_20")]
pub use self::flags::SEAT_CAPABILITY_POINTER;
#[cfg(feature = "v3_20")]
pub use self::flags::SEAT_CAPABILITY_TOUCH;
#[cfg(feature = "v3_20")]
pub use self::flags::SEAT_CAPABILITY_TABLET_STYLUS;
#[cfg(feature = "v3_20")]
pub use self::flags::SEAT_CAPABILITY_KEYBOARD;
#[cfg(feature = "v3_20")]
pub use self::flags::SEAT_CAPABILITY_ALL_POINTING;
#[cfg(feature = "v3_20")]
pub use self::flags::SEAT_CAPABILITY_ALL;
pub use self::flags::WMDecoration;
pub use self::flags::DECOR_ALL;
pub use self::flags::DECOR_BORDER;
pub use self::flags::DECOR_RESIZEH;
pub use self::flags::DECOR_TITLE;
pub use self::flags::DECOR_MENU;
pub use self::flags::DECOR_MINIMIZE;
pub use self::flags::DECOR_MAXIMIZE;
pub use self::flags::WMFunction;
pub use self::flags::FUNC_ALL;
pub use self::flags::FUNC_RESIZE;
pub use self::flags::FUNC_MOVE;
pub use self::flags::FUNC_MINIMIZE;
pub use self::flags::FUNC_MAXIMIZE;
pub use self::flags::FUNC_CLOSE;
pub use self::flags::WindowHints;
pub use self::flags::HINT_POS;
pub use self::flags::HINT_MIN_SIZE;
pub use self::flags::HINT_MAX_SIZE;
pub use self::flags::HINT_BASE_SIZE;
pub use self::flags::HINT_ASPECT;
pub use self::flags::HINT_RESIZE_INC;
pub use self::flags::HINT_WIN_GRAVITY;
pub use self::flags::HINT_USER_POS;
pub use self::flags::HINT_USER_SIZE;
pub use self::flags::WindowState;
pub use self::flags::WINDOW_STATE_WITHDRAWN;
pub use self::flags::WINDOW_STATE_ICONIFIED;
pub use self::flags::WINDOW_STATE_MAXIMIZED;
pub use self::flags::WINDOW_STATE_STICKY;
pub use self::flags::WINDOW_STATE_FULLSCREEN;
pub use self::flags::WINDOW_STATE_ABOVE;
pub use self::flags::WINDOW_STATE_BELOW;
pub use self::flags::WINDOW_STATE_FOCUSED;
pub use self::flags::WINDOW_STATE_TILED;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::AppLaunchContextExt;
    pub use super::CursorExt;
    pub use super::DeviceExt;
    pub use super::DeviceManagerExt;
    #[cfg(feature = "v3_22")]
    pub use super::DeviceToolExt;
    pub use super::DisplayExt;
    pub use super::DisplayManagerExt;
    pub use super::DragContextExt;
    #[cfg(feature = "v3_22")]
    pub use super::DrawingContextExt;
    #[cfg(feature = "v3_8")]
    pub use super::FrameClockExt;
    #[cfg(feature = "v3_16")]
    pub use super::GLContextExt;
    #[cfg(feature = "v3_22")]
    pub use super::MonitorExt;
    pub use super::ScreenExt;
    #[cfg(feature = "v3_20")]
    pub use super::SeatExt;
    pub use super::VisualExt;
    pub use super::WindowExt;
}
