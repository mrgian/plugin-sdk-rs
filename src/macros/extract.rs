#[macro_export]
macro_rules! plugin_extract {
    ($value:expr) => {
        //FIELD EXTRACTION FUNCTIONS

        //get_fields
        #[no_mangle]
        pub unsafe extern "C" fn plugin_get_fields() -> *const ::std::os::raw::c_char {
            return $value.get_fields().as_ptr() as *const ::std::os::raw::c_char;
        }

        const hello_field: &str = "Hello field!";
        const HELLO_FIELD: &str = formatcp!("{}\0", hello_field);

        //extract_fields
        #[no_mangle]
        pub unsafe extern "C" fn plugin_extract_fields(
            s: *mut ss_plugin_t,
            evt: *const ss_plugin_event_input,
            in_: *const ss_plugin_field_extract_input,
        ) -> ss_plugin_rc {
            let state = s as *mut DummyState;

            //NOTE: i still don't know why num_fields is 1 even if the field count is 2
            let num_fields = (*in_).num_fields as usize;
            let fields = std::slice::from_raw_parts_mut((*in_).fields, num_fields);

            let result = $value.extract_fields(state.as_mut().unwrap(), fields);

            //let mut ptr = HELLO_FIELD.as_ptr() as *const ::std::os::raw::c_char;

            //(*(*in_).fields).res.str_ = &mut ptr as *mut *const ::std::os::raw::c_char;
            //(*(*in_).fields).res_len = 1;

            return 0;
        }
    };
}
