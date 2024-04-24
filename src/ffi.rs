use autocxx::include_cpp;

include_cpp! {
    #include "uhal/uhal.hpp"
    name!(ffi2)
    safety!(unsafe)
    generate!("uhal::Node")
    generate!("uhal::ConnectionManager")
    generate!("uhal::HwInterface")
}

pub use ffi2::uhal::ConnectionManager;
