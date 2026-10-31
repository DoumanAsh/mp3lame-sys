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
}
