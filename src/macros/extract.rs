#[macro_export]
macro_rules! plugin_extract {
    ($value:expr) => {
        //FIELD EXTRACTION FUNCTIONS

        //get_fields
        #[no_mangle]
        pub unsafe extern "C" fn plugin_get_fields() -> *const ::std::os::raw::c_char {
            return $value.get_fields().as_ptr() as *const ::std::os::raw::c_char;
        }

        //extract_fields
        #[no_mangle]
        pub unsafe extern "C" fn plugin_extract_fields(
            s: *mut ss_plugin_t,
            evt: *const ss_plugin_event_input,
            in_: *const ss_plugin_field_extract_input,
        ) -> ss_plugin_rc {
            let mut ptr = $value.extract_fields().as_ptr() as *const ::std::os::raw::c_char;

            (*(*in_).fields).res.str_ = &mut ptr as *mut *const ::std::os::raw::c_char;
            (*(*in_).fields).res_len = 1;

            return 0;
        }

    };
}