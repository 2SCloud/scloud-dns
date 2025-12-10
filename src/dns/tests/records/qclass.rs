
#[cfg(test)]
mod tests {
    use crate::dns::records::DNSClass;

    #[test]
    fn qclass_to_u16() {
        assert_eq!(u16::from(DNSClass::NONE), 0);
        assert_eq!(u16::from(DNSClass::IN), 1);
        assert_eq!(u16::from(DNSClass::CS), 2);
        assert_eq!(u16::from(DNSClass::CH), 3);
        assert_eq!(u16::from(DNSClass::HS), 4);
        assert_eq!(u16::from(DNSClass::ANY), 255);
        assert_eq!(u16::from(DNSClass::Unknown(168)), 168);
    }

    #[test]
    fn u16_to_qclass() {
        assert_eq!(DNSClass::from(0), DNSClass::NONE);
        assert_eq!(DNSClass::from(1), DNSClass::IN);
        assert_eq!(DNSClass::from(2), DNSClass::CS);
        assert_eq!(DNSClass::from(3), DNSClass::CH);
        assert_eq!(DNSClass::from(4), DNSClass::HS);
        assert_eq!(DNSClass::from(255), DNSClass::ANY);
        assert_eq!(DNSClass::from(23), DNSClass::Unknown(23));
    }

    #[test]
    fn test_dnsclass_from_bytes() {

        let bytes = [0x00, 0x01];
        assert_eq!(DNSClass::from(&bytes), DNSClass::IN);

        let bytes = [0x00, 0x03];
        assert_eq!(DNSClass::from(&bytes), DNSClass::CH);

        let bytes = [0x00, 0xFF];
        assert_eq!(DNSClass::from(&bytes), DNSClass::ANY);

        let bytes = [0x12, 0x34];
        assert_eq!(DNSClass::from(&bytes), DNSClass::Unknown(0x1234));
    }

}