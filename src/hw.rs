use cxx::UniquePtr;

use crate::ffi;

pub struct HwInterface {
    pub(crate) ffi_class: UniquePtr<ffi::HwInterface>,
}

impl HwInterface {}
