use std::collections::{HashMap, VecDeque};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM, RECT, HMODULE};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::{HBRUSH, RedrawWindow, InvalidateRect, HRGN, RDW_INVALIDATE, RDW_UPDATENOW};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::core::PCWSTR;

use crate::embedder::{Embedder, EmbedderConfig, EmbedderError, Event, Surface};
use crate::window::{WindowConfig, WindowEmbedder, WindowId, WindowError, Window};

const WINDOW_CLASS_NAME: &str = "VectronWindowClass";

struct Win32WindowData {
    hwnd: HWND,
    title: String,
    width: u32,
    height: u32,
    parent: Option<WindowId>,
    scale_factor: f32,
}

pub struct Win32Embedder {
    instance: HMODULE,
    windows: HashMap<WindowId, Win32WindowData>,
    running: bool,
    next_handle: WindowId,
    event_queue: VecDeque<Event>,
    config: Option<EmbedderConfig>,
}

impl Win32Embedder {
    pub fn new() -> Self {
        Self {
            instance: unsafe { GetModuleHandleW(None).unwrap_or_default() },
            windows: HashMap::new(),
            running: false,
            next_handle: 1,
            event_queue: VecDeque::new(),
            config: None,
        }
    }

    fn register_window_class(&self) -> Result<(), EmbedderError> {
        unsafe {
            let class_name = WINDOW_CLASS_NAME.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
            let class_name_ptr = PCWSTR::from_raw(class_name.as_ptr());
            
            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: self.instance.into(),
                hIcon: HICON::default(),
                hCursor: LoadCursorW(Some(self.instance.into()), IDC_ARROW).unwrap_or_default(),
                hbrBackground: HBRUSH::default(),
                lpszMenuName: PCWSTR::null(),
                lpszClassName: class_name_ptr,
                hIconSm: HICON::default(),
            };

            if RegisterClassExW(&wc) == 0 {
                return Err(EmbedderError::InitializationFailed(
                    "Failed to register window class".to_string(),
                ));
            }
        }
        Ok(())
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_SIZE => {
                let _width = LOWORD(lparam.0 as u32);
                let _height = HIWORD(lparam.0 as u32);
                // Handle window resize
                LRESULT(0)
            }
            WM_MOVE => {
                let _x = GET_X_LPARAM(lparam);
                let _y = GET_Y_LPARAM(lparam);
                // Handle window move
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }

    fn get_window_scale_factor(&self, handle: WindowId) -> f32 {
        if let Some(window) = self.windows.get(&handle) {
            window.scale_factor
        } else {
            1.0
        }
    }
}

impl Embedder for Win32Embedder {
    type Handle = WindowId;

    fn init(&mut self, config: EmbedderConfig) -> Result<(), EmbedderError> {
        self.config = Some(config);
        self.register_window_class()?;
        self.running = true;
        Ok(())
    }

    fn process_events(&mut self) -> Vec<Event> {
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageW(&mut msg, Some(HWND::default()), 0, 0, PM_REMOVE).as_bool() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);

                match msg.message {
                    WM_QUIT => {
                        self.running = false;
                        self.event_queue.push_back(Event::Quit);
                    }
                    WM_SIZE => {
                        let width = LOWORD(msg.lParam.0 as u32);
                        let height = HIWORD(msg.lParam.0 as u32);
                        self.event_queue.push_back(Event::Resized { width, height });
                    }
                    WM_MOVE => {
                        let x = GET_X_LPARAM(msg.lParam);
                        let y = GET_Y_LPARAM(msg.lParam);
                        self.event_queue.push_back(Event::Moved { x, y });
                    }
                    _ => {}
                }
            }
        }

        self.event_queue.drain(..).collect()
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn get_surface(&self, handle: WindowId) -> Result<Surface, EmbedderError> {
        let window = self.windows.get(&handle).ok_or(EmbedderError::InvalidHandle)?;
        Ok(Surface {
            handle: window.hwnd.0 as *mut std::ffi::c_void,
            width: window.width,
            height: window.height,
            scale_factor: window.scale_factor,
        })
    }

    fn request_redraw(&mut self, handle: WindowId) {
        if let Some(window) = self.windows.get(&handle) {
            unsafe {
                let rect: Option<*const RECT> = None;
                InvalidateRect(Some(window.hwnd), rect, true);
            }
        }
    }

    fn shutdown(&mut self) {
        self.running = false;
        // Clean up windows
        for window in self.windows.values() {
            unsafe {
                DestroyWindow(window.hwnd);
            }
        }
        self.windows.clear();
    }
}

impl WindowEmbedder for Win32Embedder {
    fn create_window(&mut self, config: &WindowConfig) -> Result<WindowId, WindowError> {
        unsafe {
            let style = if config.decorated {
                WS_OVERLAPPEDWINDOW
            } else {
                WS_POPUP
            };

            let class_name = WINDOW_CLASS_NAME.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
            let class_name_ptr = PCWSTR::from_raw(class_name.as_ptr());
            
            let title = config.title.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
            let title_ptr = PCWSTR::from_raw(title.as_ptr());

            let x = config.position.map_or(CW_USEDEFAULT, |(x, _)| x);
            let y = config.position.map_or(CW_USEDEFAULT, |(_, y)| y);

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                class_name_ptr,
                title_ptr,
                style,
                x,
                y,
                config.width as i32,
                config.height as i32,
                Some(HWND::default()),
                Some(HMENU::default()),
                Some(self.instance.into()),
                None,
            ).unwrap_or_else(|_| HWND::default());

            if hwnd.is_invalid() {
                return Err(WindowError::CreationFailed(
                    "Failed to create window".to_string(),
                ));
            }

            let handle = self.next_handle;
            self.next_handle += 1;

            let window_data = Win32WindowData {
                hwnd,
                title: config.title.clone(),
                width: config.width,
                height: config.height,
                parent: config.parent,
                scale_factor: 1.0, // TODO: Get actual DPI scale factor
            };

            self.windows.insert(handle, window_data);

            if config.visible {
                ShowWindow(hwnd, SW_SHOW);
                RedrawWindow(Some(hwnd), None, Some(HRGN::default()), RDW_INVALIDATE | RDW_UPDATENOW);
            }

            Ok(handle)
        }
    }
    
    fn get_window(&self, handle: &WindowId) -> Option<Window> {
        self.windows.get(handle).map(|win_data| {
            Window::new(
                self.instance.0 as *mut std::ffi::c_void,
                *handle,
                win_data.hwnd.0 as *mut std::ffi::c_void,
                win_data.width,
                win_data.height,
            )
        })
    }
    
    fn destroy_window(&mut self, handle: &WindowId) {
        if let Some(window) = self.windows.remove(handle) {
            unsafe {
                let _ = DestroyWindow(window.hwnd);
            }
        }
    }
    
    fn show_window(&mut self, handle: &WindowId) {
        if let Some(window) = self.windows.get(handle) {
            unsafe {
                let _ = ShowWindow(window.hwnd, SW_SHOW);
                let _ = RedrawWindow(Some(window.hwnd), None, Some(HRGN::default()), RDW_INVALIDATE | RDW_UPDATENOW);
            }
        }
    }
    
    fn hide_window(&mut self, handle: &WindowId) {
        if let Some(window) = self.windows.get(handle) {
            unsafe {
                ShowWindow(window.hwnd, SW_HIDE);
            }
        }
    }
    
    fn set_window_title(&mut self, handle: &WindowId, title: &str) {
        if let Some(window) = self.windows.get_mut(handle) {
            unsafe {
                let title_wide = title.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                let title_ptr = PCWSTR::from_raw(title_wide.as_ptr());
                SetWindowTextW(window.hwnd, title_ptr);
            }
            window.title = title.to_string();
        }
    }
    
    fn set_window_size(&mut self, handle: &WindowId, width: u32, height: u32) {
        if let Some(window) = self.windows.get_mut(handle) {
            unsafe {
                let _ = SetWindowPos(
                    window.hwnd,
                    Some(HWND::default()),
                    0,
                    0,
                    width as i32,
                    height as i32,
                    SWP_NOMOVE | SWP_NOZORDER,
                );
            }
            window.width = width;
            window.height = height;
        }
    }
    
    fn get_window_size(&self, handle: &WindowId) -> (u32, u32) {
        if let Some(window) = self.windows.get(handle) {
            (window.width, window.height)
        } else {
            (0, 0)
        }
    }
    
    fn set_window_position(&mut self, handle: &WindowId, x: i32, y: i32) {
        if let Some(window) = self.windows.get(handle) {
            unsafe {
                let _ = SetWindowPos(
                    window.hwnd,
                    Some(HWND::default()),
                    x,
                    y,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOZORDER,
                );
            }
        }
    }
    
    fn get_window_position(&self, handle: &WindowId) -> (i32, i32) {
        if let Some(window) = self.windows.get(handle) {
            unsafe {
                let mut rect = RECT::default();
                GetWindowRect(window.hwnd, &mut rect);
                (rect.left, rect.top)
            }
        } else {
            (0, 0)
        }
    }
    
    fn set_window_parent(&mut self, handle: &WindowId, parent: &WindowId) {
        // Get parent HWND first before mutably borrowing
        let parent_hwnd = self.windows.get(parent).map(|parent_window| parent_window.hwnd);
        
        if let (Some(window), Some(parent_hwnd)) = (self.windows.get_mut(handle), parent_hwnd) {
            unsafe {
                SetParent(window.hwnd, Some(parent_hwnd));
            }
            window.parent = Some(*parent);
        }
    }
}

// Helper functions for working with window messages
#[inline]
fn LOWORD(dword: u32) -> u32 {
    dword & 0xFFFF
}

#[inline]
fn HIWORD(dword: u32) -> u32 {
    (dword >> 16) & 0xFFFF
}

#[inline]
fn GET_X_LPARAM(lparam: LPARAM) -> i32 {
    (lparam.0 & 0xFFFF) as i16 as i32
}

#[inline]
fn GET_Y_LPARAM(lparam: LPARAM) -> i32 {
    ((lparam.0 >> 16) & 0xFFFF) as i16 as i32
}
