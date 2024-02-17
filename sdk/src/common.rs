#[macro_export]
macro_rules! plugin_common {
    ($type:ty) => {
        //INFO FUNCTIONS

        //get_required_api_version
        #[no_mangle]
        pub extern "C" fn plugin_get_required_api_version() -> *const ::std::os::raw::c_char {
            return PLUGIN.required_api_version.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_name
        #[no_mangle]
        pub extern "C" fn plugin_get_name() -> *const ::std::os::raw::c_char {
            return PLUGIN.name.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_description
        #[no_mangle]
        pub extern "C" fn plugin_get_description() -> *const ::std::os::raw::c_char {
            return PLUGIN.description.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_contact
        #[no_mangle]
        pub extern "C" fn plugin_get_contact() -> *const ::std::os::raw::c_char {
            return PLUGIN.contact.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_version
        #[no_mangle]
        pub extern "C" fn plugin_get_version() -> *const ::std::os::raw::c_char {
            return PLUGIN.version.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_event_source
        #[no_mangle]
        pub extern "C" fn plugin_get_event_source() -> *const ::std::os::raw::c_char {
            return PLUGIN.event_source.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_id
        #[no_mangle]
        pub extern "C" fn plugin_get_id() -> u32 {
            return PLUGIN.id;
        }

        //get_last_error
        #[no_mangle]
        pub unsafe extern "C" fn plugin_get_last_error(
            s: *mut ss_plugin_t,
        ) -> *const ::std::os::raw::c_char {
            //let state = Box::from_raw(s as *mut PluginState);
            //let error_ptr = (*state).last_error.as_ptr();
            return std::ptr::null() as *const ::std::os::raw::c_char;
            //return error_ptr as *const ::std::os::raw::c_char;
        }

        //INSTANCE/CAPTURE MANAGEMENT FUNCTIONS

        //init
        #[no_mangle]
        pub unsafe extern "C" fn plugin_init(
            input: *const ss_plugin_init_input,
            rc: *mut ss_plugin_rc,
        ) -> *mut ss_plugin_t {
            *rc = 0;
            let mut raw = Box::into_raw(PLUGIN.init());
            return raw as *mut ss_plugin_t;
        }

        //destroy
        #[no_mangle]
        pub unsafe extern "C" fn plugin_destroy(s: *mut ss_plugin_t) {
            let state = s as *mut $type;
            PLUGIN.destroy(state.as_mut().unwrap());
        }
    };
}
