use libweston_sys::{
    weston_view, weston_view_create, weston_view_destroy,
    weston_layer_entry
};
use super::surface::Surface;

pub struct View {
    ptr: *mut weston_view,
}

impl View {
    pub fn new(surface: &Surface) -> View {
        View {
            ptr: unsafe { weston_view_create(surface.ptr()) }
        }
    }

    prop_accessors!(weston_layer_entry | layer_link);

    pub fn ptr(&self) -> *mut weston_view {
        self.ptr
    }
}

// impl Drop for View {
//     fn drop(&mut self) {
//         unsafe { weston_view_destroy(self.ptr); }
//     }
// }