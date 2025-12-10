
#[cfg(test)]
mod tests {
    use std::result;
    use crate::dns::records::q_name::{parse_qname, parse_qname_at};

    #[test]
    fn test_parse_qname() {

        let bytes: &[u8] = &[
            0xAA,0xAA,
            0x01,
            0x00,0x00,0x01,0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x04,0x72,0x75,0x73,0x74,
            0x06,0x74,0x72,0x65,0x6E,0x64,0x73,
            0x03,0x63,0x6F,0x6D,
            0x00,
            0x00,0x01,
            0x00,0x01];

        let qname_bytes = &bytes[12..]; // start at QNAME
        let (qname, consumed) = parse_qname(qname_bytes).unwrap();
        println!("expected: rust.trends.com\ngot: {}", qname);
        assert_eq!(consumed, 17);
        assert_eq!(qname, "rust.trends.com");
    }

    #[test]
    fn test_parse_qname_at() {
        let bytes: &[u8] = &[
            0xAA,0xAA,
            0x01,
            0x00,0x00,0x01,0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x04,0x72,0x75,0x73,0x74,
            0x06,0x74,0x72,0x65,0x6E,0x64,0x73,
            0x03,0x63,0x6F,0x6D,
            0x00,
            0x00,0x01,
            0x00,0x01];

        let qname = parse_qname_at(bytes, 12).unwrap();
        println!("expected: rust.trends.com\ngot: {:?}", qname);
        assert_eq!(qname, "rust.trends.com");
    }

}