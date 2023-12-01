#[macro_export]
macro_rules! plugin_common {
    ($value:expr) => {
        //INFO FUNCTIONS

        //get_required_api_version
        const required_api_version: &str = $value.required_api_version;
        const REQUIRED_API_VERSION: &str = formatcp!("{}\0", required_api_version);

        #[no_mangle]
        pub extern "C" fn plugin_get_required_api_version() -> *const ::std::os::raw::c_char {
            return REQUIRED_API_VERSION.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_name
        const name: &str = $value.name;
        const NAME: &str = formatcp!("{}\0", name);

        #[no_mangle]
        pub extern "C" fn plugin_get_name() -> *const ::std::os::raw::c_char {
            return NAME.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_description
        const description: &str = $value.description;
        const DESCRIPTION: &str = formatcp!("{}\0", description);

        #[no_mangle]
        pub extern "C" fn plugin_get_description() -> *const ::std::os::raw::c_char {
            return DESCRIPTION.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_contact
        const contact: &str = $value.contact;
        const CONTACT: &str = formatcp!("{}\0", contact);

        #[no_mangle]
        pub extern "C" fn plugin_get_contact() -> *const ::std::os::raw::c_char {
            return CONTACT.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_version
        const version: &str = $value.version;
        const VERSION: &str = formatcp!("{}\0", version);

        #[no_mangle]
        pub extern "C" fn plugin_get_version() -> *const ::std::os::raw::c_char {
            return VERSION.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_event_source
        const event_source: &str = $value.event_source;
        const EVENT_SOURCE: &str = formatcp!("{}\0", event_source);

        #[no_mangle]
        pub extern "C" fn plugin_get_event_source() -> *const ::std::os::raw::c_char {
            return EVENT_SOURCE.as_ptr() as *const ::std::os::raw::c_char;
        }

        //get_id
        #[no_mangle]
        pub extern "C" fn plugin_get_id() -> u32 {
            return $value.id;
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
            let mut raw = Box::into_raw($value.init());
            return raw as *mut ss_plugin_t;
        }

        //destroy
        #[no_mangle]
        pub unsafe extern "C" fn plugin_destroy(s: *mut ss_plugin_t) {
            let state = s as *mut PluginState;
            $value.destroy(state.as_mut().unwrap());
        }
    };
}
