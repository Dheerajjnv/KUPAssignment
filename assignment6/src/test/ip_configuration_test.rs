#[cfg(test)]
#[test]
fn class_a_success() {
    use crate::ip_configuration;
    use crate::ip_configuration::IpAddress;

    let test_output: IpAddress = ip_configuration::ip_address_classification((20, 74, 52, 32));
    assert_eq!(test_output, IpAddress::_ClassA(String::from("20.74.52.32")));
}

#[test]
fn class_b_success() {
    use crate::ip_configuration;
    use crate::ip_configuration::IpAddress;
    let test_output: IpAddress = ip_configuration::ip_address_classification((152, 174, 152, 132));
    assert_eq!(
        test_output,
        IpAddress::_ClassB(String::from("152.174.152.132"))
    );
}

#[test]
fn class_c_success() {
    use crate::ip_configuration;
    use crate::ip_configuration::IpAddress;
    let test_output: IpAddress = ip_configuration::ip_address_classification((201, 204, 202, 211));
    assert_eq!(
        test_output,
        IpAddress::_ClassC(String::from("201.204.202.211"))
    );
}
#[test]
fn class_d_success() {
    use crate::ip_configuration;
    use crate::ip_configuration::IpAddress;
    let test_output: IpAddress = ip_configuration::ip_address_classification((233, 234, 232, 232));
    assert_eq!(
        test_output,
        IpAddress::_ClassD(String::from("233.234.232.232"))
    );
}

#[test]
fn invalid_success() {
    use crate::ip_configuration;
    use crate::ip_configuration::IpAddress;
    let test_output: IpAddress = ip_configuration::ip_address_classification((320, 374, 352, 432));
    assert_eq!(
        test_output,
        IpAddress::_InvalidConfiguration(String::from("Invalid ip_configuration"))
    );
}
