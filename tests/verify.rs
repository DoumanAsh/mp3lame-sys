use mp3lame_sys::*;

use core::mem::MaybeUninit;

#[cfg(feature = "decoder")]
#[test]
fn should_decode() {
    struct Decoder {
        hip: hip_t
    }

    impl Decoder {
        fn new() -> Self {
            let hip = unsafe {
                hip_decode_init()
            };
            assert!(!hip.is_null());
            Self {
                hip
            }
        }
    }

    impl Drop for Decoder {
        fn drop(&mut self) {
            unsafe {
                hip_decode_exit(self.hip);
            }
        }
    }

    Decoder::new();
}

#[test]
fn should_test_version() {
    let mut version = MaybeUninit::<lame_version_t>::uninit();
    let version = unsafe {
        get_lame_version_numerical(version.as_mut_ptr());
        version.assume_init()
    };

    assert_eq!(version.major, 3);
    assert_eq!(version.minor, 100);
}


