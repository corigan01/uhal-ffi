#[allow(unused_imports)]
pub mod ffi;

#[cfg(test)]
mod test {
    use super::ffi::ConnectionManager;
    use crate::ffi;
    use autocxx::WithinUniquePtr;
    use cxx::let_cxx_string;

    #[test]
    fn test_stuffs() {
        let_cxx_string!(
            filename = "file://ipbus-software/uhal/tests/etc/uhal/tests/dummy_connections.xml"
        );
        let_cxx_string!(devicename = "dummy.udp");
        let_cxx_string!(node_test = "REG");

        let mut connection = ConnectionManager::new1(&filename).within_unique_ptr();
        let mut hw = connection
            .pin_mut()
            .getDevice(&devicename)
            .within_unique_ptr();
        let hw = hw.pin_mut();

        let node = hw.get_node(&node_test);
        let value = node.read();

        hw.dispatch();

        panic!("Value: {}", ffi::value(&value));
    }
}
