use std::num::ParseIntError;
use custom_error::custom_error;
use crate::constants::{UNDEF_CIDR, MAX_CIDR};

custom_error!{
    /// Describes a parsing error of some kind
    pub ParseError
        GenericError{position: String, value: String} = "Error parsing value in {position}. It was '{value}'",
        MaxCidrExceeded{value: u8} = "Maximum CIDR value exceeded. It was {value}"
}

custom_error!{
    /// Describes an error related with a SubnetMask type
    pub NetmaskError
        UndefinedCidr = "Undefinded CIDR, cannot proceed",
        MaxCidrExceeded{value: u8} = "Maximum CIDR value exceeded. It was {value}",
        CalculationError = "Unable to calculate netmask due to previous error"
}

/// Represents a single IP address
#[derive(Clone, Copy)]
pub struct IPAddress {
    /// First byte of IP address
    pub b0: u8,
    /// Second byte of IP address
    pub b1: u8,
    /// Third byte of IP address
    pub b2: u8,
    /// Fourth byte of IP address
    pub b3: u8,
    /// CIDR value of IP address
    pub cidr: u8,
}

/// Represents a dot.decimal notation subnet mask
#[derive(Clone, Copy)]
pub struct SubnetMask {
    /// First byte of IP address
    pub b0: u8,
    /// Second byte of IP address
    pub b1: u8,
    /// Third byte of IP address
    pub b2: u8,
    /// Fourth byte of IP address
    pub b3: u8,
}

impl IPAddress {
    /// Creates a new IP address struct
    /// 
    /// Parameters:
    /// * `b0`: first byte of IP address
    /// * `b1`: second byte of IP address
    /// * `b2`: third byte of IP address
    /// * `b3`: fourth byte of IP address
    /// * `cidr`: CIDR value of IP address
    pub fn new(b0: u8, b1: u8, b2: u8, b3: u8, cidr: u8) -> IPAddress {
        IPAddress { b0, b1, b2, b3, cidr }
    }

    /// Creates a new IP address struct
    /// 
    /// Parameters:
    /// * `b0`: first byte of IP address
    /// * `b1`: second byte of IP address
    /// * `b2`: third byte of IP address
    /// * `b3`: fourth byte of IP address
    pub fn new_without_cidr(b0: u8, b1: u8, b2: u8, b3: u8) -> IPAddress {
        IPAddress::new(b0, b1, b2, b3, UNDEF_CIDR)
    }

    /// Construct an IP address from string parameter
    /// 
    /// Parameters:
    /// * `ip_address`: String value with IP address. It may or may not contain CIDR value.
    pub fn from_string(ip_address: String) -> Result<IPAddress, ParseError> {
        // IP string separators
        const SEP: [char; 2] = ['.', '/'];

        // Split string into chunks
        let chunks: Vec<&str> = ip_address.split(&SEP).collect();

        // Try to parse all chunks
        let b0 = chunks[0].parse();
        if b0.is_err() {
            return Err(ParseError::GenericError { position: "byte 0".to_string(), value: chunks[0].to_string() })
        }

        let b1 = chunks[1].parse();
        if b1.is_err() {
            return Err(ParseError::GenericError { position: "byte 1".to_string(), value: chunks[1].to_string() })
        }

        let b2 = chunks[2].parse();
        if b2.is_err() {
            return Err(ParseError::GenericError { position: "byte 2".to_string(), value: chunks[2].to_string() })
        }

        let b3 = chunks[3].parse();
        if b3.is_err() {
            return Err(ParseError::GenericError { position: "byte 3".to_string(), value: chunks[3].to_string() })
        }

        let mut cidr = UNDEF_CIDR;

        // Check if we have to parse CIDR or not
        if chunks.len() >= 5 { 
            let v_cidr: Result<u8, ParseIntError> = chunks[4].parse();
            if v_cidr.is_err() {
                return Err(ParseError::GenericError { position: "CIDR value".to_string(), value: chunks[4].to_string() })
            }

            // Check if CIDR does not exceed max allowed value
            if v_cidr.as_ref().unwrap() > &MAX_CIDR {
                return Err(ParseError::MaxCidrExceeded { value: v_cidr.unwrap() });
            }

            cidr = v_cidr.unwrap()
        }


        Ok(IPAddress::new(b0.unwrap(), b1.unwrap(), b2.unwrap(), b3.unwrap(), cidr))
    }

    /// Constructs an IP address from string slice
    /// 
    /// Parameters:
    /// * `ip_address`: string slice value with IP address. It may or may not contain CIDR value.
    pub fn from_str(ip_address: &str) -> Result<IPAddress, ParseError> {
        IPAddress::from_string(ip_address.to_string())
    }

    /// Converts an IP address into a standard formatted string (dot.decimal + CIDR)
    pub fn to_string(&self) -> String {
        if self.cidr != UNDEF_CIDR {
            format!("{}.{}.{}.{}/{}", self.b0, self.b1, self.b2, self.b3, self.cidr)
        } else {
            format!("{}.{}.{}.{}", self.b0, self.b1, self.b2, self.b3)
        }
    }

    /// Calculates the subnet associated with this IP address
    pub fn calculate_subnet(&self) -> Result<IPAddress, NetmaskError> {
        let netmask = SubnetMask::from_cidr(self.cidr);
        if netmask.is_err() {
            return Err(netmask.err().unwrap());
        }

        let netmask = netmask.unwrap();
        let mut result = *self;

        result.b0 &= netmask.b0;
        result.b1 &= netmask.b1;
        result.b2 &= netmask.b2;
        result.b3 &= netmask.b3;

        Ok(result)
    }
}

impl SubnetMask {
    /// Constructs a new SubnetMask
    /// 
    /// Parametes:
    /// * `b0`: first byte of netmask
    /// * `b1`: second byte of netmask
    /// * `b2`: third byte of netmask
    /// * `b3`: fourth byte of netmask
    pub fn new(b0: u8, b1: u8, b2: u8, b3: u8) -> SubnetMask {
        SubnetMask { b0, b1, b2, b3 }
    }

    /// Constructs a new SubnetMask given a CIDR value
    /// 
    /// Parameters:
    /// * `cidr`: CIDR decimal value
    pub fn from_cidr(cidr: u8) -> Result<SubnetMask, NetmaskError> {
        if cidr == UNDEF_CIDR {
            return Err(NetmaskError::UndefinedCidr);
        }

        if cidr > MAX_CIDR {
            return Err(NetmaskError::MaxCidrExceeded { value: cidr })
        }

        let mut val = SubnetMask::new(0, 0, 0, 0);

        // Set bits for masking
        let mut bits = 0_usize;
        for i in MAX_CIDR - cidr..MAX_CIDR {
            bits |= 1 << i;
        }

        val.b0 = ((bits & 0xFF000000) >> 24) as u8;
        val.b1 = ((bits & 0xFF0000) >> 16) as u8;
        val.b2 = ((bits & 0xFF00) >> 8) as u8;
        val.b3 = (bits & 0xFF) as u8;

        Ok(val)
    }

    /// Constructs a subnet mask from string
    /// 
    /// Parameters:
    /// * `netmask`: String value of subnet mask
    pub fn from_string(netmask: String) -> Result<SubnetMask, ParseError> {
        const SEP: char = '.';
        
        // Split into chunks
        let chunks: Vec<&str> = netmask.split(SEP).collect();

        // Parse chunks and set values
        let b0 = chunks[0].parse();
        if b0.is_err() {
            return Err(ParseError::GenericError { position: "byte 0".to_string(), value: chunks[0].to_string() })
        }

        let b1 = chunks[1].parse();
        if b1.is_err() {
            return Err(ParseError::GenericError { position: "byte 1".to_string(), value: chunks[1].to_string() })
        }

        let b2 = chunks[2].parse();
        if b2.is_err() {
            return Err(ParseError::GenericError { position: "byte 2".to_string(), value: chunks[2].to_string() })
        }

        let b3 = chunks[3].parse();
        if b3.is_err() {
            return Err(ParseError::GenericError { position: "byte 3".to_string(), value: chunks[3].to_string() })
        }

        Ok(SubnetMask::new(b0.unwrap(), b1.unwrap(), b2.unwrap(), b3.unwrap()))
    }

    /// Constructs a subnet mask from string
    /// 
    /// Parameters:
    /// * `netmask`: string slice value of subnet mask
    pub fn from_str(netmask: &str) -> Result<SubnetMask, ParseError> {
        SubnetMask::from_string(netmask.to_string())
    }

    /// Returns a human readable dot.decimal string of this Subnet mask
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}.{}", self.b0, self.b1, self.b2, self.b3)
    }
}

/// Counts the number of set bits
/// 
/// Parameters:
/// * `b0`: first byte of IP address
/// * `b1`: second byte of IP address
/// * `b2`: third byte of IP address
/// * `b3`: fourth byte of IP address
fn count_set_bits(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    b0.count_ones() + b1.count_ones() + b2.count_ones() + b3.count_ones()
}
