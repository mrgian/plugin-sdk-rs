#[macro_export]
macro_rules! plugin_source {
    ($value:expr) => {
        //EVENTS FUNCTIONS

        static mut event: Event = Event {
            ts: u64::MAX,
            tid: u64::MAX,
            len: 55,
            type_: 322,
            nparams: 2,
            reserved: 4,
            data_len: 0,
            plugin_id: 999,
            //data: [0; 32],
        };

        //open
        #[no_mangle]
        pub unsafe extern "C" fn plugin_open(
            s: *mut ss_plugin_t,
            params: *const ::std::os::raw::c_char,
            rc: *mut ss_plugin_rc,
        ) -> *mut ss_instance_t {
            let state = s as *mut PluginState;
            let result = $value.open(state.as_mut().unwrap());

            match result {
                Ok(()) => {
                    *rc = 0;
                }
                Err(error) => {
                    //todo: set error string
                }
            }

            return std::ptr::null_mut();
        }

        //close
        #[no_mangle]
        pub unsafe extern "C" fn plugin_close(s: *mut ss_plugin_t, h: *mut ss_instance_t) {
            let state = s as *mut PluginState;
            $value.close(state.as_mut().unwrap());
        }

        //next_batch
        #[no_mangle]
        pub unsafe extern "C" fn plugin_next_batch(
            s: *mut ss_plugin_t,
            h: *mut ss_instance_t,
            nevts: *mut u32,
            evts: *mut *mut *mut ss_plugin_event,
        ) -> ss_plugin_rc {
            let state = s as *mut PluginState;
            let result = $value.next_batch(state.as_mut().unwrap());

            match result {
                Ok(rc) => {
                    match rc {
                        0 => *nevts = 1,
                        _ => *nevts = 0,
                    }

                    let mut event_ptr = (&mut event as *mut Event) as *mut ss_plugin_event;
                    *evts = &mut event_ptr as *mut *mut ss_plugin_event;

                    return rc;
                }
                Err(erro) => {
                    return 1;
                }
            }
        }
    };
}
