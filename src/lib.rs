#[allow(unused_imports)]
pub mod ffi;

#[cfg(test)]
mod test {
    use super::ffi::ConnectionManager;
    use autocxx::WithinUniquePtr;
    use cxx::let_cxx_string;

    #[test]
    fn test_stuffs() {
        let_cxx_string!(
            filename = "file://ipbus-software/uhal/tests/etc/uhal/tests/dummy_connections.xml"
        );
        let_cxx_string!(devicename = "dummy.udp");

        let connection = ConnectionManager::new1(&filename).within_unique_ptr();

        panic!("{:#?}", connection.getDevices1(&devicename));
    }
}
