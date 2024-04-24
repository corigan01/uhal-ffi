use autocxx::include_cpp;
use cxx::CxxString;

include_cpp! {
    #include "uhal/uhal.hpp"
    name!(autoffi)
    safety!(unsafe)
    generate!("uhal::Node")
    generate!("uhal::ConnectionManager")
    generate!("uhal::HwInterface")
    generate!("uhal::ValWord")
    generate!("uhal::ValVector")
    generate!("uhal::ValHeader")
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

pub use autoffi::uhal::ConnectionManager;
pub use autoffi::uhal::HwInterface;
pub use autoffi::uhal::Node;
pub use autoffi::uhal::ValHeader;
pub use autoffi::uhal::ValVector;
pub use autoffi::uhal::ValWord;
