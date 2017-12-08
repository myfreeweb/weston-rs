use std::ptr;
use libc;
use libweston_sys::{
    weston_compositor, weston_compositor_create, weston_compositor_destroy,
    weston_compositor_shutdown,
    weston_compositor_set_xkb_rule_names, xkb_rule_names,
    weston_compositor_wake, weston_compositor_schedule_repaint,
    weston_pending_output_coldplug
};
use wayland_sys::server::wl_signal;
use ::display::Display;

pub struct Compositor {
    ptr: *mut weston_compositor,
}

impl Compositor {
    pub fn new(display: &Display) -> Compositor {
        let mut result = Compositor {
            ptr: unsafe { weston_compositor_create(display.ptr(), ptr::null_mut()) },
        };
        // TODO check ptr != null
        unsafe { (*result.ptr).user_data = &mut result as *mut _ as *mut libc::c_void };
        result
    }

    pub fn set_xkb_rule_names(&mut self, names: Option<*mut xkb_rule_names>) {
        unsafe { weston_compositor_set_xkb_rule_names(self.ptr, names.unwrap_or(ptr::null_mut())); }
    }

    pub fn schedule_repaint(&self) {
        unsafe { weston_compositor_schedule_repaint(self.ptr); }
    }

    pub fn pending_output_coldplug(&self) {
        unsafe { weston_pending_output_coldplug(self.ptr); }
    }

    pub fn wake(&self) {
        unsafe { weston_compositor_wake(self.ptr); }
    }

    pub fn shutdown(&mut self) {
        unsafe { weston_compositor_shutdown(self.ptr); }
    }

    prop_accessors!(
        wl_signal | destroy_signal, create_surface_signal, activate_signal, transform_signal,
        kill_signal, idle_signal, wake_signal, show_input_panel_signal, hide_input_panel_signal,
        update_input_panel_signal, seat_created_signal, output_pending_signal, output_created_signal,
        output_destroyed_signal, output_moved_signal, output_resized_signal, session_signal);

    pub fn ptr(&self) -> *mut weston_compositor {
        self.ptr
    }
}

impl Drop for Compositor {
    fn drop(&mut self) {
        unsafe { weston_compositor_destroy(self.ptr); }
    }
}
