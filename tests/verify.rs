use mp3lame_sys::*;

use core::mem::MaybeUninit;

#[test]
fn should_test_version() {
    let mut version = MaybeUninit::<lame_version_t>::uninit();
    let version = unsafe {
        get_lame_version_numerical(version.as_mut_ptr());
        version.assume_init()
    };

    assert_eq!(version.major, 4);
    assert_eq!(version.minor, 0);
    let features = unsafe {
        core::ffi::CStr::from_ptr(version.features)
    };
    assert_eq!(features.to_str().unwrap(), "");


    let bitness = unsafe {
        core::ffi::CStr::from_ptr(get_lame_os_bitness()).to_str().expect("to parse bitness")
    };

    #[cfg(target_pointer_width = "64")]
    assert_eq!(bitness, "64bits");
    #[cfg(not(target_pointer_width = "64"))]
    assert_eq!(bitness, "32bits");
}
