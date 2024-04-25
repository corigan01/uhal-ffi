use autocxx::include_cpp;
use cxx::CxxString;

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

pub use autoffi::ready;
pub use autoffi::uhal::ConnectionManager;
pub use autoffi::uhal::HwInterface;
pub use autoffi::uhal::Node;
pub use autoffi::uhal::ValHeader;
pub use autoffi::value;
pub use autoffi::ValWord32;
