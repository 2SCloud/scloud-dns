#[cfg(test)]
mod tests {
    use crate::dns::records::q_name::{parse_qname, parse_qname_at};
    use crate::exceptions::SCloudException;

    #[test]
    fn test_parse_qname() {
        let bytes: &[u8] = &[
            0xAA, 0xAA, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x72,
            0x75, 0x73, 0x74, 0x06, 0x74, 0x72, 0x65, 0x6E, 0x64, 0x73, 0x03, 0x63, 0x6F, 0x6D,
            0x00, 0x00, 0x01, 0x00, 0x01,
        ];

        let qname_bytes = &bytes[12..]; // start at QNAME
        let (qname, consumed) = parse_qname(qname_bytes, 0).unwrap();
        println!("expected: rust.trends.com\ngot: {}", qname);
        assert_eq!(consumed, 17);
        assert_eq!(qname, "rust.trends.com");
    }

    #[test]
    fn test_parse_qname_at() {
        let bytes: &[u8] = &[
            0xAA, 0xAA, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x72,
            0x75, 0x73, 0x74, 0x06, 0x74, 0x72, 0x65, 0x6E, 0x64, 0x73, 0x03, 0x63, 0x6F, 0x6D,
            0x00, 0x00, 0x01, 0x00, 0x01,
        ];

        let qname = parse_qname_at(bytes, 12).unwrap();
        println!("expected: rust.trends.com\ngot: {:?}", qname);
        assert_eq!(qname, "rust.trends.com");
    }

    #[test]
    fn test_pos_superior_to_buf_len() {
        let bytes: &[u8] = &[0x01, 0x01];
        let result = parse_qname(bytes, 0);
        println!(
            "expected: SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF\ngot: {:?}",
            parse_qname(bytes, 0)
        );
        assert!(matches!(
            result,
            Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF)
        ));
    }

    #[test]
    fn test_pos_and_len_superior_to_buf_len() {
        let buf = [5, b'a', b'b'];
        let result = parse_qname(&buf, 0);

        assert!(matches!(
            result,
            Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF)
        ));
    }

    #[test]
    fn test_compression_pointer_ok() {
        let mut buf = vec![
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ];

        let compressed_offset = buf.len();
        buf.extend_from_slice(&[0xC0, 0x00]);

        let name = parse_qname_at(&buf, compressed_offset).unwrap();

        assert_eq!(name, "www.google.com");
    }

    #[test]
    fn test_compression_pointer_out_of_bounds() {
        let buf = vec![0xC0];

        let result = parse_qname(&buf, 0);
        assert_eq!(
            result.unwrap_err(),
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME
        );
    }
}
