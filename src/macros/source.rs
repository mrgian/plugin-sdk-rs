#[macro_export]
macro_rules! plugin_source {
    ($value:expr) => {
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
    }
}