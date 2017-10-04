#![allow(bad_style)]

extern crate libc;

use libc::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ushort, c_void};
use libc::{uint32_t, uint64_t};

#[cfg(target_os = "windows")]
#[link(name = "opengl32")]
#[link(name = "gdi32")]
#[link(name = "user32")]
extern "C" {}

#[cfg(target_os = "linux")]
#[link(name = "X11")]
#[link(name = "Xrandr")]
#[link(name = "Xinerama")]
#[link(name = "Xxf86vm")]
#[link(name = "Xcursor")]
#[link(name = "dl")]
#[link(name = "GL")]
#[link(name = "m")]
#[link(name = "pthread")]
#[link(name = "rt")]
extern "C" {}

pub enum Monitor {}
pub enum Window {}
pub enum Cursor {}

pub type Glproc = *const std::os::raw::c_void;
pub type Errorfun = extern "C" fn(error: c_int, description: *const c_char);
pub type Monitorfun = extern "C" fn(monitor: *mut Monitor, event: c_int);
pub type Framebuffersizefun = extern "C" fn(window: *mut Window, width: c_int, height: c_int);
pub type Windowclosefun = extern "C" fn(window: *mut Window);
pub type Windowfocusfun = extern "C" fn(window: *mut Window, focused: c_int);
pub type Windowiconifyfun = extern "C" fn(window: *mut Window, iconified: c_int);
pub type Windowposfun = extern "C" fn(window: *mut Window, xpos: c_int, ypos: c_int);
pub type Windowrefreshfun = extern "C" fn(window: *mut Window);
pub type Windowsizefun = extern "C" fn(window: *mut Window, width: c_int, height: c_int);
pub type Charfun = extern "C" fn(window: *mut Window, codepoint: c_uint);
pub type Charmodsfun = extern "C" fn(window: *mut Window, codepoint: c_uint, mods: c_int);
pub type Cursorenterfun = extern "C" fn(window: *mut Window, entered: c_int);
pub type Cursorposfun = extern "C" fn(window: *mut Window, xpos: c_double, ypos: c_double);
pub type Dropfun = extern "C" fn(window: *mut Window, count: c_int, paths: *const *const c_char);
pub type Joystickfun = extern "C" fn(joy: c_int, event: c_int);
pub type Keyfun = extern "C" fn(
    window: *mut Window,
    key: c_int,
    scancode: c_int,
    action: c_int,
    mods: c_int,
);
pub type Mousebuttonfun = extern "C" fn(
    window: *mut Window,
    button: c_int,
    action: c_int,
    mods: c_int,
);
pub type Scrollfun = extern "C" fn(window: *mut Window, xoffset: c_double, yoffset: c_double);

// Vulkan, subject to change
pub enum VkInstance {}
pub enum VkPhysicalDevice {}
pub enum VkAllocationCallbacks {}
pub enum VkSurfaceKHR {}
pub type Vkproc = *const std::os::raw::c_void;
pub type VkResult = c_int;

#[repr(C)]
pub struct Vidmode {
    pub width: c_int,
    pub height: c_int,
    pub redBits: c_int,
    pub greenBits: c_int,
    pub blueBits: c_int,
    pub refreshRate: c_int,
}

#[repr(C)]
pub struct Gammaramp {
    pub red: *mut c_ushort,
    pub green: *mut c_ushort,
    pub blue: *mut c_ushort,
    pub size: c_uint,
}

#[repr(C)]
pub struct Image {
    pub width: c_int,
    pub height: c_int,
    pub pixels: *mut c_uchar,
}

// Input
pub const KEY_UNKNOWN: c_int = -1;
pub const KEY_SPACE: c_int = 32;
pub const KEY_APOSTROPHE: c_int = 39;
pub const KEY_COMMA: c_int = 44;
pub const KEY_MINUS: c_int = 45;
pub const KEY_PERIOD: c_int = 46;
pub const KEY_SLASH: c_int = 47;
pub const KEY_0: c_int = 48;
pub const KEY_1: c_int = 49;
pub const KEY_2: c_int = 50;
pub const KEY_3: c_int = 51;
pub const KEY_4: c_int = 52;
pub const KEY_5: c_int = 53;
pub const KEY_6: c_int = 54;
pub const KEY_7: c_int = 55;
pub const KEY_8: c_int = 56;
pub const KEY_9: c_int = 57;
pub const KEY_SEMICOLON: c_int = 59;
pub const KEY_EQUAL: c_int = 61;
pub const KEY_A: c_int = 65;
pub const KEY_B: c_int = 66;
pub const KEY_C: c_int = 67;
pub const KEY_D: c_int = 68;
pub const KEY_E: c_int = 69;
pub const KEY_F: c_int = 70;
pub const KEY_G: c_int = 71;
pub const KEY_H: c_int = 72;
pub const KEY_I: c_int = 73;
pub const KEY_J: c_int = 74;
pub const KEY_K: c_int = 75;
pub const KEY_L: c_int = 76;
pub const KEY_M: c_int = 77;
pub const KEY_N: c_int = 78;
pub const KEY_O: c_int = 79;
pub const KEY_P: c_int = 80;
pub const KEY_Q: c_int = 81;
pub const KEY_R: c_int = 82;
pub const KEY_S: c_int = 83;
pub const KEY_T: c_int = 84;
pub const KEY_U: c_int = 85;
pub const KEY_V: c_int = 86;
pub const KEY_W: c_int = 87;
pub const KEY_X: c_int = 88;
pub const KEY_Y: c_int = 89;
pub const KEY_Z: c_int = 90;
pub const KEY_LEFT_BRACKET: c_int = 91;
pub const KEY_BACKSLASH: c_int = 92;
pub const KEY_RIGHT_BRACKET: c_int = 93;
pub const KEY_GRAVE_ACCENT: c_int = 96;
pub const KEY_WORLD_1: c_int = 161;
pub const KEY_WORLD_2: c_int = 162;
pub const KEY_ESCAPE: c_int = 256;
pub const KEY_ENTER: c_int = 257;
pub const KEY_TAB: c_int = 258;
pub const KEY_BACKSPACE: c_int = 259;
pub const KEY_INSERT: c_int = 260;
pub const KEY_DELETE: c_int = 261;
pub const KEY_RIGHT: c_int = 262;
pub const KEY_LEFT: c_int = 263;
pub const KEY_DOWN: c_int = 264;
pub const KEY_UP: c_int = 265;
pub const KEY_PAGE_UP: c_int = 266;
pub const KEY_PAGE_DOWN: c_int = 267;
pub const KEY_HOME: c_int = 268;
pub const KEY_END: c_int = 269;
pub const KEY_CAPS_LOCK: c_int = 280;
pub const KEY_SCROLL_LOCK: c_int = 281;
pub const KEY_NUM_LOCK: c_int = 282;
pub const KEY_PRINT_SCREEN: c_int = 283;
pub const KEY_PAUSE: c_int = 284;
pub const KEY_F1: c_int = 290;
pub const KEY_F2: c_int = 291;
pub const KEY_F3: c_int = 292;
pub const KEY_F4: c_int = 293;
pub const KEY_F5: c_int = 294;
pub const KEY_F6: c_int = 295;
pub const KEY_F7: c_int = 296;
pub const KEY_F8: c_int = 297;
pub const KEY_F9: c_int = 298;
pub const KEY_F10: c_int = 299;
pub const KEY_F11: c_int = 300;
pub const KEY_F12: c_int = 301;
pub const KEY_F13: c_int = 302;
pub const KEY_F14: c_int = 303;
pub const KEY_F15: c_int = 304;
pub const KEY_F16: c_int = 305;
pub const KEY_F17: c_int = 306;
pub const KEY_F18: c_int = 307;
pub const KEY_F19: c_int = 308;
pub const KEY_F20: c_int = 309;
pub const KEY_F21: c_int = 310;
pub const KEY_F22: c_int = 311;
pub const KEY_F23: c_int = 312;
pub const KEY_F24: c_int = 313;
pub const KEY_F25: c_int = 314;
pub const KEY_KP_0: c_int = 320;
pub const KEY_KP_1: c_int = 321;
pub const KEY_KP_2: c_int = 322;
pub const KEY_KP_3: c_int = 323;
pub const KEY_KP_4: c_int = 324;
pub const KEY_KP_5: c_int = 325;
pub const KEY_KP_6: c_int = 326;
pub const KEY_KP_7: c_int = 327;
pub const KEY_KP_8: c_int = 328;
pub const KEY_KP_9: c_int = 329;
pub const KEY_KP_DECIMAL: c_int = 330;
pub const KEY_KP_DIVIDE: c_int = 331;
pub const KEY_KP_MULTIPLY: c_int = 332;
pub const KEY_KP_SUBTRACT: c_int = 333;
pub const KEY_KP_ADD: c_int = 334;
pub const KEY_KP_ENTER: c_int = 335;
pub const KEY_KP_EQUAL: c_int = 336;
pub const KEY_LEFT_SHIFT: c_int = 340;
pub const KEY_LEFT_CONTROL: c_int = 341;
pub const KEY_LEFT_ALT: c_int = 342;
pub const KEY_LEFT_SUPER: c_int = 343;
pub const KEY_RIGHT_SHIFT: c_int = 344;
pub const KEY_RIGHT_CONTROL: c_int = 345;
pub const KEY_RIGHT_ALT: c_int = 346;
pub const KEY_RIGHT_SUPER: c_int = 347;
pub const KEY_MENU: c_int = 348;
pub const KEY_LAST: c_int = KEY_MENU;
pub const MOD_SHIFT: c_int = 0x0001;
pub const MOD_CONTROL: c_int = 0x0002;
pub const MOD_ALT: c_int = 0x0004;
pub const MOD_SUPER: c_int = 0x0008;
pub const MOUSE_BUTTON_1: c_int = 0;
pub const MOUSE_BUTTON_2: c_int = 1;
pub const MOUSE_BUTTON_3: c_int = 2;
pub const MOUSE_BUTTON_4: c_int = 3;
pub const MOUSE_BUTTON_5: c_int = 4;
pub const MOUSE_BUTTON_6: c_int = 5;
pub const MOUSE_BUTTON_7: c_int = 6;
pub const MOUSE_BUTTON_8: c_int = 7;
pub const MOUSE_BUTTON_LAST: c_int = MOUSE_BUTTON_8;
pub const MOUSE_BUTTON_LEFT: c_int = MOUSE_BUTTON_1;
pub const MOUSE_BUTTON_RIGHT: c_int = MOUSE_BUTTON_2;
pub const MOUSE_BUTTON_MIDDLE: c_int = MOUSE_BUTTON_3;
pub const JOYSTICK_1: c_int = 0;
pub const JOYSTICK_2: c_int = 1;
pub const JOYSTICK_3: c_int = 2;
pub const JOYSTICK_4: c_int = 3;
pub const JOYSTICK_5: c_int = 4;
pub const JOYSTICK_6: c_int = 5;
pub const JOYSTICK_7: c_int = 6;
pub const JOYSTICK_8: c_int = 7;
pub const JOYSTICK_9: c_int = 8;
pub const JOYSTICK_10: c_int = 9;
pub const JOYSTICK_11: c_int = 10;
pub const JOYSTICK_12: c_int = 11;
pub const JOYSTICK_13: c_int = 12;
pub const JOYSTICK_14: c_int = 13;
pub const JOYSTICK_15: c_int = 14;
pub const JOYSTICK_16: c_int = 15;
pub const JOYSTICK_LAST: c_int = JOYSTICK_16;
pub const STICKY_KEYS: c_int = 0x00033002;
pub const STICKY_MOUSE_BUTTONS: c_int = 0x00033003;
pub const RELEASE: c_int = 0;
pub const PRESS: c_int = 1;
pub const REPEAT: c_int = 2;

// Error codes
pub const NOT_INITIALIZED: c_int = 0x00010001;
pub const NO_CURRENT_CONTEXT: c_int = 0x00010002;
pub const INVALID_ENUM: c_int = 0x00010003;
pub const INVALID_VALUE: c_int = 0x00010004;
pub const OUT_OF_MEMORY: c_int = 0x00010005;
pub const API_UNAVAILABLE: c_int = 0x00010006;
pub const VERSION_UNAVAILABLE: c_int = 0x00010007;
pub const PLATFORM_ERROR: c_int = 0x00010008;
pub const FORMAT_UNAVAILABLE: c_int = 0x00010009;
pub const NO_WINDOW_CONTEXT: c_int = 0x0001000A;

// Windows
pub const FOCUSED: c_int = 0x00020001;
pub const ICONIFIED: c_int = 0x00020002;
pub const RESIZABLE: c_int = 0x00020003;
pub const VISIBLE: c_int = 0x00020004;
pub const DECORATED: c_int = 0x00020005;
pub const AUTO_ICONIFY: c_int = 0x00020006;
pub const FLOATING: c_int = 0x00020007;
pub const MAXIMIZED: c_int = 0x00020008;

// OpenGL info
pub const RED_BITS: c_int = 0x00021001;
pub const GREEN_BITS: c_int = 0x00021002;
pub const BLUE_BITS: c_int = 0x00021003;
pub const ALPHA_BITS: c_int = 0x00021004;
pub const DEPTH_BITS: c_int = 0x00021005;
pub const STENCIL_BITS: c_int = 0x00021006;
pub const ACCUM_RED_BITS: c_int = 0x00021007;
pub const ACCUM_GREEN_BITS: c_int = 0x00021008;
pub const ACCUM_BLUE_BITS: c_int = 0x00021009;
pub const ACCUM_ALPHA_BITS: c_int = 0x0002100A;
pub const AUX_BUFFERS: c_int = 0x0002100B;
pub const STEREO: c_int = 0x0002100C;
pub const SAMPLES: c_int = 0x0002100D;
pub const SRGB_CAPABLE: c_int = 0x0002100E;
pub const REFRESH_RATE: c_int = 0x0002100F;
pub const DOUBLEBUFFER: c_int = 0x00021010;
pub const CLIENT_API: c_int = 0x00022001;
pub const CONTEXT_VERSION_MAJOR: c_int = 0x00022002;
pub const CONTEXT_VERSION_MINOR: c_int = 0x00022003;
pub const CONTEXT_REVISION: c_int = 0x00022004;
pub const CONTEXT_ROBUSTNESS: c_int = 0x00022005;
pub const OPENGL_FORWARD_COMPAT: c_int = 0x00022006;
pub const OPENGL_DEBUG_CONTEXT: c_int = 0x00022007;
pub const OPENGL_PROFILE: c_int = 0x00022008;
pub const CONTEXT_RELEASE_BEHAVIOR: c_int = 0x00022009;
pub const CONTEXT_NO_ERROR: c_int = 0x0002200A;
pub const CONTEXT_CREATION_API: c_int = 0x0002200B;
pub const NO_API: c_int = 0;
pub const OPENGL_API: c_int = 0x00030001;
pub const OPENGL_ES_API: c_int = 0x00030002;
pub const NO_ROBUSTNESS: c_int = 0;
pub const NO_RESET_NOTIFICATION: c_int = 0x00031001;
pub const LOSE_CONTEXT_ON_RESET: c_int = 0x00031002;
pub const OPENGL_ANY_PROFILE: c_int = 0;
pub const OPENGL_CORE_PROFILE: c_int = 0x00032001;
pub const OPENGL_COMPAT_PROFILE: c_int = 0x00032002;
pub const ANY_RELEASE_BEHAVIOR: c_int = 0;
pub const RELEASE_BEHAVIOR_FLUSH: c_int = 0x00035001;
pub const RELEASE_BEHAVIOR_NONE: c_int = 0x00035002;
pub const NATIVE_CONTEXT_API: c_int = 0x00036001;
pub const EGL_CONTEXT_API: c_int = 0x00036002;
pub const CONNECTED: c_int = 0x00040001;
pub const DISCONNECTED: c_int = 0x00040002;
pub const DONT_CARE: c_int = -1;

// Cursors
pub const ARROW_CURSOR: c_int = 0x00036001;
pub const IBEAM_CURSOR: c_int = 0x00036002;
pub const CROSSHAIR_CURSOR: c_int = 0x00036003;
pub const HAND_CURSOR: c_int = 0x00036004;
pub const HRESIZE_CURSOR: c_int = 0x00036005;
pub const VRESIZE_CURSOR: c_int = 0x00036006;
pub const CURSOR_NORMAL: c_int = 0x00034001;
pub const CURSOR_HIDDEN: c_int = 0x00034002;
pub const CURSOR_DISABLED: c_int = 0x00034003;

// Others
pub const TRUE: c_int = 1;
pub const FALSE: c_int = 0;

extern "C" {
    #[link_name = "glfwInit"]
    pub fn Init() -> c_int;
    #[link_name = "glfwTerminate"]
    pub fn Terminate();
    #[link_name = "glfwGetVersion"]
    pub fn GetVersion(major: *mut c_int, minor: *mut c_int, rev: *mut c_int);
    #[link_name = "glfwGetVersionString"]
    pub fn GetVersionString() -> *const c_char;
    #[link_name = "glfwSetErrorCallback"]
    pub fn SetErrorCallback(cbfun: Errorfun) -> Errorfun;
    #[link_name = "glfwCreateCursor"]
    pub fn CreateCursor(image: *const Image, xhot: c_int, yhot: c_int) -> *mut Cursor;
    #[link_name = "glfwCreateStandardCursor"]
    pub fn CreateStandardCursor(shape: c_int) -> *mut Cursor;
    #[link_name = "glfwDestroyCursor"]
    pub fn DestroyCursor(cursor: *mut Cursor);
    #[link_name = "glfwPollEvents"]
    pub fn PollEvents();
    #[link_name = "glfwWaitEvents"]
    pub fn WaitEvents();
    #[link_name = "glfwWaitEventsTimeout"]
    pub fn WaitEventsTimeout(timeout: c_double);
    #[link_name = "glfwPostEmptyEvent"]
    pub fn PostEmptyEvent();
    #[link_name = "glfwGetKeyName"]
    pub fn GetKeyName(key: c_int, scancode: c_int) -> *const c_char;
    #[link_name = "glfwJoystickPresent"]
    pub fn JoystickPresent(joy: c_int) -> c_int;
    #[link_name = "glfwGetJoystickAxes"]
    pub fn GetJoystickAxes(joy: c_int, count: *mut c_int) -> *const c_float;
    #[link_name = "glfwGetJoystickButtons"]
    pub fn GetJoystickButtons(joy: c_int, count: *mut c_int) -> *const c_uchar;
    #[link_name = "glfwGetJoystickName"]
    pub fn GetJoystickName(joy: c_int) -> *const c_char;
    #[link_name = "glfwSetJoystickCallback"]
    pub fn SetJoystickCallback(cbfun: Joystickfun) -> Joystickfun;
    #[link_name = "glfwGetTime"]
    pub fn GetTime() -> c_double;
    #[link_name = "glfwSetTime"]
    pub fn SetTime(time: c_double);
    #[link_name = "glfwGetTimerValue"]
    pub fn GetTimerValue() -> uint64_t;
    #[link_name = "glfwGetTimerFrequency"]
    pub fn GetTimerFrequency() -> uint64_t;
    #[link_name = "glfwSwapInterval"]
    pub fn SwapInterval(interval: c_int);
    #[link_name = "glfwExtensionSupported"]
    pub fn ExtensionSupported(extension: *const c_char) -> c_int;
    #[link_name = "glfwGetProcAddress"]
    pub fn GetProcAddress(procname: *const c_char) -> Glproc;

    // Monitors
    #[link_name = "glfwGetMonitors"]
    pub fn GetMonitors(count: *mut c_int) -> *mut *mut Monitor;
    #[link_name = "glfwGetPrimaryMonitor"]
    pub fn GetPrimaryMonitor() -> *mut Monitor;
    #[link_name = "glfwGetMonitorPos"]
    pub fn GetMonitorPos(monitor: *mut Monitor, xpos: *mut c_int, ypos: *mut c_int);
    #[link_name = "glfwGetMonitorPhysicalSize"]
    pub fn GetMonitorPhysicalSize(monitor: *mut Monitor, widthMM: *mut c_int, heightMM: *mut c_int);
    #[link_name = "glfwGetMonitorName"]
    pub fn GetMonitorName(monitor: *mut Monitor) -> *const c_char;
    #[link_name = "glfwSetMonitorCallback"]
    pub fn SetMonitorCallback(cbfun: Monitorfun) -> Monitorfun;
    #[link_name = "glfwGetVideoModes"]
    pub fn GetVideoModes(monitor: *mut Monitor, count: *mut c_int) -> *const Vidmode;
    #[link_name = "glfwGetVideoMode"]
    pub fn GetVideoMode(monitor: *mut Monitor) -> *const Vidmode;

    // Gamma
    #[link_name = "glfwSetGamma"]
    pub fn SetGamma(monitor: *mut Monitor, gamma: c_float);
    #[link_name = "glfwGetGammaRamp"]
    pub fn GetGammaRamp(monitor: *mut Monitor) -> *const Gammaramp;
    #[link_name = "glfwSetGammaRamp"]
    pub fn SetGammaRamp(monitor: *mut Monitor, ramp: *const Gammaramp);

    // Windows
    #[link_name = "glfwDefaultWindowHints"]
    pub fn DefaultWindowHints();
    #[link_name = "glfwWindowHint"]
    pub fn WindowHint(hint: c_int, value: c_int);
    #[link_name = "glfwCreateWindow"]
    pub fn CreateWindow(
        width: c_int,
        height: c_int,
        title: *const c_char,
        monitor: *mut Monitor,
        share: *mut Window,
    ) -> *mut Window;
    #[link_name = "glfwDestroyWindow"]
    pub fn DestroyWindow(window: *mut Window);
    #[link_name = "glfwWindowShouldClose"]
    pub fn WindowShouldClose(window: *mut Window) -> c_int;
    #[link_name = "glfwSetWindowShouldClose"]
    pub fn SetWindowShouldClose(window: *mut Window, value: c_int);
    #[link_name = "glfwSetWindowTitle"]
    pub fn SetWindowTitle(wndow: *mut Window, title: *const c_char);
    #[link_name = "glfwSetWindowIcon"]
    pub fn SetWindowIcon(window: *mut Window, count: c_int, images: *const Image);
    #[link_name = "glfwGetWindowPos"]
    pub fn GetWindowPos(window: *mut Window, xpos: *mut c_int, ypos: *mut c_int);
    #[link_name = "glfwSetWindowPos"]
    pub fn SetWindowPos(window: *mut Window, xpos: c_int, ypos: c_int);
    #[link_name = "glfwGetWindowSize"]
    pub fn GetWindowSize(window: *mut Window, width: *mut c_int, height: *mut c_int);
    #[link_name = "glfwSetWindowSizeLimits"]
    pub fn SetWindowSizeLimits(
        window: *mut Window,
        minwidth: c_int,
        min_height: c_int,
        maxwidth: c_int,
        maxheight: c_int,
    );
    #[link_name = "glfwSetWindowAspectRation"]
    pub fn SetWindowAspectRation(window: *mut Window, numer: c_int, denom: c_int);
    #[link_name = "glfwSetWindowSize"]
    pub fn SetWindowSize(window: *mut Window, width: c_int, height: c_int);
    #[link_name = "glfwGetFramebufferSize"]
    pub fn GetFramebufferSize(window: *mut Window, width: c_int, height: c_int);
    #[link_name = "glfwGetWindowFrameSize"]
    pub fn GetWindowFrameSize(
        window: *mut Window,
        left: *mut c_int,
        top: *mut c_int,
        right: *mut c_int,
        bottom: *mut c_int,
    );
    #[link_name = "glfwIconifyWindow"]
    pub fn IconifyWindow(window: *mut Window);
    #[link_name = "glfwRestoreWindow"]
    pub fn RestoreWindow(window: *mut Window);
    #[link_name = "glfwMaximizeWindow"]
    pub fn MaximizeWindow(window: *mut Window);
    #[link_name = "glfwShowWindow"]
    pub fn ShowWindow(window: *mut Window);
    #[link_name = "glfwHideWindow"]
    pub fn HideWindow(window: *mut Window);
    #[link_name = "glfwFocusWindow"]
    pub fn FocusWindow(window: *mut Window);
    #[link_name = "glfwGetWindowMonitor"]
    pub fn GetWindowMonitor(window: *mut Window) -> *mut Monitor;
    #[link_name = "glfwSetWindowMonitor"]
    pub fn SetWindowMonitor(
        window: *mut Window,
        monitor: *mut Monitor,
        xpos: c_int,
        ypos: c_int,
        width: c_int,
        height: c_int,
        refreshRate: c_int,
    );
    #[link_name = "glfwGetWindowAttrib"]
    pub fn GetWindowAttrib(window: *mut Window, attrib: c_int);
    #[link_name = "glfwSetWindowUserPointer"]
    pub fn SetWindowUserPointer(window: *mut Window, pointer: *mut c_void);
    #[link_name = "glfwGetWindowUserPointer"]
    pub fn GetWindowUserPointer(window: *mut Window) -> *mut c_void;
    #[link_name = "glfwSetWindowPosCallback"]
    pub fn SetWindowPosCallback(window: *mut Window, cbfun: Windowposfun) -> Windowposfun;
    #[link_name = "glfwSetWindowSizeCallback"]
    pub fn SetWindowSizeCallback(window: *mut Window, cbfun: Windowsizefun) -> Windowsizefun;
    #[link_name = "glfwSetWindowCloseCallback"]
    pub fn SetWindowCloseCallback(window: *mut Window, cbfun: Windowclosefun) -> Windowclosefun;
    #[link_name = "glfwSetWindowRefreshCallback"]
    pub fn SetWindowRefreshCallback(
        window: *mut Window,
        cbfun: Windowrefreshfun,
    ) -> Windowrefreshfun;
    #[link_name = "glfwSetWindowFocusCallback"]
    pub fn SetWindowFocusCallback(window: *mut Window, cbfun: Windowfocusfun) -> Windowfocusfun;
    #[link_name = "glfwSetWindowIconifyCallback"]
    pub fn SetWindowIconifyCallback(
        window: *mut Window,
        cbfun: Windowiconifyfun,
    ) -> Windowiconifyfun;
    #[link_name = "glfwSetFramebufferSizeCallback"]
    pub fn SetFramebufferSizeCallback(
        window: *mut Window,
        cbfun: Framebuffersizefun,
    ) -> Framebuffersizefun;
    #[link_name = "glfwGetInputMode"]
    pub fn GetInputMode(window: *mut Window, mode: c_int) -> c_int;
    #[link_name = "glfwSetInputMode"]
    pub fn SetInputMode(window: *mut Window, mode: c_int, value: c_int);
    #[link_name = "glfwGetKey"]
    pub fn GetKey(window: *mut Window, key: c_int) -> c_int;
    #[link_name = "glfwGetMouseButton"]
    pub fn GetMouseButton(window: *mut Window, button: c_int) -> c_int;
    #[link_name = "glfwGetCursorPos"]
    pub fn GetCursorPos(window: *mut Window, xpos: *mut c_double, ypos: *mut c_double);
    #[link_name = "glfwSetCursorPos"]
    pub fn SetCursorPos(window: *mut Window, xpos: c_double, ypos: c_double);
    #[link_name = "glfwSetCursor"]
    pub fn SetCursor(window: *mut Window, cursor: *mut Cursor);
    #[link_name = "glfwSetKeyCallback"]
    pub fn SetKeyCallback(window: *mut Window, cbfun: Keyfun) -> Keyfun;
    #[link_name = "glfwSetCharCallback"]
    pub fn SetCharCallback(window: *mut Window, cbfun: Charfun) -> Charfun;
    #[link_name = "glfwSetCharModsCallback"]
    pub fn SetCharModsCallback(window: *mut Window, cbfun: Charmodsfun) -> Charmodsfun;
    #[link_name = "glfwSetMouseButtonCallback"]
    pub fn SetMouseButtonCallback(window: *mut Window, cbfun: Mousebuttonfun) -> Mousebuttonfun;
    #[link_name = "glfwSetCursorPosCallback"]
    pub fn SetCursorPosCallback(window: *mut Window, cbfun: Cursorposfun) -> Cursorposfun;
    #[link_name = "glfwSetCursorEnterCallback"]
    pub fn SetCursorEnterCallback(window: *mut Window, cbfun: Cursorenterfun) -> Cursorenterfun;
    #[link_name = "glfwSetScrollCallback"]
    pub fn SetScrollCallback(window: *mut Window, cbfun: Scrollfun) -> Scrollfun;
    #[link_name = "glfwSetDropCallback"]
    pub fn SetDropCallback(window: *mut Window, cbfun: Dropfun) -> Dropfun;
    #[link_name = "glfwSetClipboardString"]
    pub fn SetClipboardString(window: *mut Window, string: *const c_char);
    #[link_name = "glfwGetClipboardString"]
    pub fn GetClipboardString(window: *mut Window) -> *const c_char;
    #[link_name = "glfwMakeContextCurrent"]
    pub fn MakeContextCurrent(window: *mut Window);
    #[link_name = "glfwGetCurrentContext"]
    pub fn GetCurrentContext() -> *mut Window;
    #[link_name = "glfwSwapBuffers"]
    pub fn SwapBuffers(window: *mut Window);

    // Vulkan, subject to change
    #[link_name = "glfwVulkanSupported"]
    pub fn VulkanSupported() -> c_int;
    #[link_name = "glfwGetRequiredInstanceExtensions"]
    pub fn GetRequiredInstanceExtensions(count: *mut uint32_t) -> *const *const c_char;
    #[link_name = "glfwGetInstanceProcAddress"]
    pub fn GetInstanceProcAddress(instance: VkInstance, procname: *const c_char) -> Vkproc;
    #[link_name = "glfwGetPhysicalDevicePresentationSupport"]
    pub fn GetPhysicalDevicePresentationSupport(
        instance: VkInstance,
        device: VkPhysicalDevice,
        queuefamily: uint32_t,
    ) -> c_int;
    #[link_name = "glfwCreateWindowSurface"]
    pub fn CreateWindowSurface(
        instance: VkInstance,
        window: *mut Window,
        allocator: *const VkAllocationCallbacks,
        surface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
