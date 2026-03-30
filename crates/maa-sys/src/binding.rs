#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_attributes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use std::path::PathBuf;

    use crate::binding::MaaCore;

    fn get_test_lib() -> MaaCore {
        let path = PathBuf::from(std::env::var("MAA_LIB_PATH").expect("MAA_LIB_PATH env not set"));
        
        // 如果路径是文件，直接使用；如果是目录，根据 target_os 拼接库文件名
        let lib_path = if path.is_file() {
            path
        } else {
            path.join(match std::env::consts::OS {
                "macos" => "libMaaCore.dylib",
                "windows" => "MaaCore.dll",
                "linux" => "libMaaCore.so",
                _ => panic!("Unsupported OS"),
            })
        };
        
        unsafe { MaaCore::new(lib_path).expect("Failed to load MaaCore library") }
    }

    #[test]
    fn test_asst_get_null_size() {
        unsafe {
            let lib = get_test_lib();
            let ret = lib.AsstGetNullSize();
            // -1 in cpp, which becomes u64::MAX when cast to unsigned
            let expected = u64::MAX;
            assert_eq!(ret, expected);
        }
    }

    #[test]
    fn test_asst_get_version() {
        unsafe {
            let lib = get_test_lib();
            let version_ptr = lib.AsstGetVersion();
            let version = CStr::from_ptr(version_ptr).to_str().unwrap_or("unknown");
            assert_ne!(version, "unknown");
        }
    }
}
