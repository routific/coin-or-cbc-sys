extern crate coin_or_cbc_sys as cbc;

use std::ffi::CString;

#[test]
fn read_and_solve() {
    let model: *mut cbc::Cbc_Model = unsafe { cbc::Cbc_newModel() };

    let filename = CString::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/p0033.mps"))
        .expect("Failed to build CString");
    let nb_errors = unsafe { cbc::Cbc_readMps(model, filename.as_ptr()) };
    assert_eq!(nb_errors, 0);

    unsafe {
        cbc::Cbc_solve(model);
        assert_eq!(cbc::Cbc_isProvenOptimal(model), 1);
        cbc::Cbc_deleteModel(model);
    }
}
