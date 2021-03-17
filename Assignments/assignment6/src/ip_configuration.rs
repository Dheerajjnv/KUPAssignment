// #Variants
//ClassA, ClassB, ClassC, ClassD, InvalidConfiguration.
#[derive(PartialEq, Eq, Debug)]
pub enum IpAddress {
    _ClassA(String),
    _ClassB(String),
    _ClassC(String),
    _ClassD(String),
    _InvalidConfiguration(String),
}
// ip_address_classification  Function classify the ip_configuration.
//
// #Arguments
// ip_address-Argument of tuple type.
//
// #Return.
// IpAddress-Return the Class of IpAddress type.
pub fn ip_address_classification(ip_address: (i32, i32, i32, i32)) -> IpAddress {
    match ip_address {
        (a, _, _, _) if (0..128).contains(&a) => IpAddress::_ClassA(format!(
            "{}.{}.{}.{}",
            ip_address.0, ip_address.1, ip_address.2, ip_address.3
        )),
        (a, _, _, _) if (128..191).contains(&a) => IpAddress::_ClassB(format!(
            "{}.{}.{}.{}",
            ip_address.0, ip_address.1, ip_address.2, ip_address.3
        )),
        (a, _, _, _) if (191..223).contains(&a) => IpAddress::_ClassC(format!(
            "{}.{}.{}.{}",
            ip_address.0, ip_address.1, ip_address.2, ip_address.3
        )),
        (a, _, _, _) if (223..239).contains(&a) => IpAddress::_ClassD(format!(
            "{}.{}.{}.{}",
            ip_address.0, ip_address.1, ip_address.2, ip_address.3
        )),
        _ => IpAddress::_InvalidConfiguration(String::from("Invalid ip_configuration")),
    }
}
