pub mod constants;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{types::{IPAddress, SubnetMask}, constants::UNDEF_CIDR};

    #[test]
    fn ip_from_string_with_cidr() {
        let address = "192.168.1.2/24".to_string();
        let ip = IPAddress::from_string(address.to_string());
        
        // Assert that it did not fail
        assert!(ip.is_ok());

        // If it did not fail, now check values
        let ip = ip.unwrap();

        assert_eq!(192, ip.b0);
        assert_eq!(168, ip.b1);
        assert_eq!(1, ip.b2);
        assert_eq!(2, ip.b3);
        assert_eq!(24, ip.cidr);
    }

    #[test]
    fn wrong_ip_from_string_with_cidr() {
        let address = "192.i68.1.2/24".to_string();

        let ip = IPAddress::from_string(address.to_string());
        
        // Assert that it failed
        assert!(ip.is_err());
    }

    #[test]
    fn ip_from_string_without_cidr() {
        let address = "192.168.1.2".to_string();
        let ip = IPAddress::from_string(address.to_string());
        
        // Assert that it did not fail
        assert!(ip.is_ok());

        // If it did not fail, now check values
        let ip = ip.unwrap();

        assert_eq!(192, ip.b0);
        assert_eq!(168, ip.b1);
        assert_eq!(1, ip.b2);
        assert_eq!(2, ip.b3);
        assert_eq!(UNDEF_CIDR, ip.cidr);
    }

    #[test]
    fn ip_from_str_slice_with_cidr() {
        let address = "192.168.1.2/24";
        let ip = IPAddress::from_str(address);
        // Assert for failure
        assert!(ip.is_ok());

        // If it did not fail, now check values
        let ip = ip.unwrap();

        assert_eq!(192, ip.b0);
        assert_eq!(168, ip.b1);
        assert_eq!(1, ip.b2);
        assert_eq!(2, ip.b3);
        assert_eq!(24, ip.cidr);
    }

    #[test]
    fn ip_from_str_slice_without_cidr() {
        let address = "192.168.1.2";
        let ip = IPAddress::from_str(address);
        // Assert for failure
        assert!(ip.is_ok());

        // If it did not fail, now check values
        let ip = ip.unwrap();

        assert_eq!(192, ip.b0);
        assert_eq!(168, ip.b1);
        assert_eq!(1, ip.b2);
        assert_eq!(2, ip.b3);
        assert_eq!(UNDEF_CIDR, ip.cidr);
    }

    #[test]
    fn ip_to_string_without_cidr() {
        let ip = IPAddress::new_without_cidr(192, 168, 1, 243);
        let string = ip.to_string();

        assert_eq!(string, "192.168.1.243");
    }

    #[test]
    fn ip_to_string_with_cidr() {
        let ip = IPAddress::new(192, 168, 1, 243, 24);
        let string = ip.to_string();

        assert_eq!(string, "192.168.1.243/24");
    }

    #[test]
    fn netmask_from_string() {
        let netmask = "255.255.0.0".to_string();
        let nm = SubnetMask::from_string(netmask);

        // Assert that it did not fail
        assert!(nm.is_ok());

        // Now check values
        let nm = nm.unwrap();
        assert_eq!(255, nm.b0);
        assert_eq!(255, nm.b1);
        assert_eq!(0, nm.b2);
        assert_eq!(0, nm.b3);
    }

    #[test]
    fn netmask_from_str() {
        let netmask = "255.255.0.0";
        let nm = SubnetMask::from_str(netmask);

        // Assert that it did not fail
        assert!(nm.is_ok());

        // Now check values
        let nm = nm.unwrap();
        assert_eq!(255, nm.b0);
        assert_eq!(255, nm.b1);
        assert_eq!(0, nm.b2);
        assert_eq!(0, nm.b3);
    }
}