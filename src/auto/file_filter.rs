// This file was generated by gir (34ea1b9) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use FileFilterFlags;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct FileFilter(Object<ffi::GtkFileFilter>): Buildable;

    match fn {
        get_type => || ffi::gtk_file_filter_get_type(),
    }
}

impl FileFilter {
    pub fn new() -> FileFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_file_filter_new())
        }
    }

    //pub fn add_custom(&self, needed: FileFilterFlags, func: /*Unknown conversion*/Unknown rust type: "FileFilterFunc", data: Fundamental: Pointer, notify: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_file_filter_add_custom() }
    //}

    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            ffi::gtk_file_filter_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    pub fn add_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_file_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    pub fn add_pixbuf_formats(&self) {
        unsafe {
            ffi::gtk_file_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    //pub fn filter(&self, filter_info: /*Ignored*/&FileFilterInfo) -> bool {
    //    unsafe { TODO: call ffi::gtk_file_filter_filter() }
    //}

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_file_filter_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_needed(&self) -> FileFilterFlags {
        unsafe {
            ffi::gtk_file_filter_get_needed(self.to_glib_none().0)
        }
    }

    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_file_filter_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }
}
