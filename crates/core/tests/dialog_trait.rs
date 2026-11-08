//! Exit code mapping tests.

use core::dialog::Exit;

#[test]
fn exit_mapping() {
    assert_eq!(Exit::Ok.into_code(), 0);
    assert_eq!(Exit::Cancel.into_code(), 1);
    assert_eq!(Exit::Timeout.into_code(), 5);
    assert_eq!(Exit::Err255.into_code(), 255);
    assert_eq!(u8::from(Exit::Ok), 0);
    assert_eq!(u8::from(Exit::Err255), 255);
}
