#[macro_export]
macro_rules! plugin {
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
        pub extern "C" fn plugin_get_last_error(
            s: *mut ss_plugin_t,
        ) -> *const ::std::os::raw::c_char {
            //return LAST_ERROR.as_ptr() as *const ::std::os::raw::c_char;
            return std::ptr::null_mut();
        }

        //INSTANCE/CAPTURE MANAGEMENT FUNCTIONS

        //init
        #[no_mangle]
        pub unsafe extern "C" fn plugin_init(
            input: *const ss_plugin_init_input,
            rc: *mut ss_plugin_rc,
        ) -> *mut ss_plugin_t {
            $value.init();
            return std::ptr::null_mut();
        }

        //destroy
        #[no_mangle]
        pub extern "C" fn plugin_destroy(s: *mut ss_plugin_t) {
            $value.destroy();
        }

        //open
        #[no_mangle]
        pub unsafe extern "C" fn plugin_open(
            s: *mut ss_plugin_t,
            params: *const ::std::os::raw::c_char,
            rc: *mut ss_plugin_rc,
        ) -> *mut ss_instance_t {
            let result = $value.open();

            match result {
                Ok(()) => {
                    *rc = 0;
                },
                Err(error) => {
                    //todo: set error string
                }
            }

            return std::ptr::null_mut();;
        }

        //close
        #[no_mangle]
        pub extern "C" fn plugin_close(s: *mut ss_plugin_t, h: *mut ss_instance_t) {
            $value.close();
        }

        //EVENTS FUNCTIONS

        //next_batch
        #[no_mangle]
        pub unsafe extern "C" fn plugin_next_batch(
            s: *mut ss_plugin_t,
            h: *mut ss_instance_t,
            nevts: *mut u32,
            evts: *mut *mut *mut ss_plugin_event,
        ) -> ss_plugin_rc {
            let result = $value.next_batch();

            match result {
                Ok(rc) => {
                    match rc {
                        0 => *nevts = 1,
                        _ => *nevts = 0,
                    }

                    let mut event_ptr = (&mut event as *mut Event) as *mut ss_plugin_event;    
                    *evts = &mut event_ptr as *mut *mut ss_plugin_event;

                    return rc;
                },
                Err(erro) => {
                    return 1;
                }
            }
        }
    };
}
