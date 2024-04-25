use autocxx::include_cpp;
use cxx::CxxString;
use cxx::UniquePtr;

include_cpp! {
    #include "uhal/uhal.hpp"
    #include "valword32.hpp"
    name!(autoffi)
    safety!(unsafe)
    concrete!("uhal::ValWord<uint32_t>", ValWord32)
    generate!("uhal::Node")
    generate!("uhal::ConnectionManager")
    generate!("uhal::HwInterface")
    generate!("uhal::ValHeader")
    generate!("ready")
    generate!("value")
}

#[cxx::bridge(namespace = "uhal")]
pub mod rawbind {
    unsafe extern "C++" {
        include!("uhal/HwInterface.hpp");

        type HwInterface = crate::ffi::HwInterface;
        type Node = crate::ffi::Node;

        #[rust_name = "get_node"]
        fn getNode<'a>(self: &'a HwInterface, aId: &CxxString) -> &'a Node;
    }
}

#[cxx::bridge(namespace = "uhal")]
pub mod resultbind {
    unsafe extern "C++" {
        include!("../extra-cpp/result.hpp");

        type ConnectionManager = crate::ffi::ConnectionManager;
        type HwInterface = crate::ffi::HwInterface;

        fn new_connection_manager_result(str: &CxxString) -> Result<UniquePtr<ConnectionManager>>;
        fn get_device_from_connection_manager_result(
            cm: &mut UniquePtr<ConnectionManager>,
            string: &CxxString,
        ) -> Result<UniquePtr<HwInterface>>;
    }
}

pub use autoffi::ready;
pub use autoffi::uhal::ConnectionManager;
pub use autoffi::uhal::HwInterface;
pub use autoffi::uhal::Node;
pub use autoffi::uhal::ValHeader;
pub use autoffi::value;
pub use autoffi::ValWord32;
