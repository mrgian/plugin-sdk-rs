#[macro_export]
macro_rules! plugin_extract {
    ($type:ty) => {
        //FIELD EXTRACTION FUNCTIONS

        //get_fields
        #[no_mangle]
        pub unsafe extern "C" fn plugin_get_fields() -> *const ::std::os::raw::c_char {
            return PLUGIN.get_fields().as_ptr() as *const ::std::os::raw::c_char;
        }

        //extract_fields
        #[no_mangle]
        pub unsafe extern "C" fn plugin_extract_fields(
            s: *mut ss_plugin_t,
            evt: *const ss_plugin_event_input,
            in_: *mut ss_plugin_field_extract_input,
        ) -> ss_plugin_rc {
            let state = s as *mut $type;

            let num_fields = (*in_).num_fields as usize;
            let fields = std::slice::from_raw_parts_mut((*in_).fields, num_fields);
            let result = PLUGIN.extract_fields(state.as_mut().unwrap(), fields);

            return 0;
        }
    };
}
