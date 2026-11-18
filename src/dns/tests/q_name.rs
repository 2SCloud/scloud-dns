#[cfg(test)]
mod tests {
    use crate::dns::q_name::parse_qname;
    use crate::dns::q_name::parse_qname_at;
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

    #[test]
    fn test_compression_pointer_chain_ok() {
        // "com" at 0, "google" + pointer to 0 at 5, "www" + pointer to 5 at 14
        let buf = vec![
            3, b'c', b'o', b'm', 0, //
            6, b'g', b'o', b'o', b'g', b'l', b'e', 0xC0, 0x00, //
            3, b'w', b'w', b'w', 0xC0, 0x05,
        ];

        let (name, end_pos) = parse_qname(&buf, 14).unwrap();

        assert_eq!(name, "www.google.com");
        assert_eq!(end_pos, 20);
    }

    #[test]
    fn test_compression_pointer_to_itself() {
        // DNS header, then a QNAME that is a pointer to its own offset (12).
        // Before the fix, parsing it recursed until the stack overflowed.
        let mut buf = vec![
            0xAA, 0xAA, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        buf.extend_from_slice(&[0xC0, 0x0C, 0x00, 0x01, 0x00, 0x01]);

        let result = parse_qname(&buf, 12);

        assert_eq!(
            result.unwrap_err(),
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED
        );
    }

    #[test]
    fn test_compression_pointers_pointing_to_each_other() {
        let buf = vec![0xC0, 0x02, 0xC0, 0x00];

        let result = parse_qname(&buf, 2);

        assert_eq!(
            result.unwrap_err(),
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED
        );
    }

    #[test]
    fn test_compression_pointer_back_into_same_name() {
        // Label "a", then a pointer to the start of the same name
        let buf = vec![1, b'a', 0xC0, 0x00];

        let result = parse_qname(&buf, 0);

        assert_eq!(
            result.unwrap_err(),
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED
        );
    }

    #[test]
    fn test_compression_pointer_forward() {
        let buf = vec![0xC0, 0x02, 3, b'c', b'o', b'm', 0];

        let result = parse_qname(&buf, 0);

        assert_eq!(
            result.unwrap_err(),
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED
        );
    }

    #[test]
    fn test_reserved_label_type() {
        let buf = vec![0x40, b'a', 0];

        let result = parse_qname(&buf, 0);

        assert_eq!(
            result.unwrap_err(),
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME
        );
    }

    #[test]
    fn test_name_at_max_length() {
        // 3 labels of 63 bytes + 1 label of 61 bytes + terminating zero = 255 bytes
        let mut buf = Vec::new();
        for len in [63u8, 63, 63, 61] {
            buf.push(len);
            buf.extend(std::iter::repeat(b'a').take(len as usize));
        }
        buf.push(0);

        let (name, end_pos) = parse_qname(&buf, 0).unwrap();

        assert_eq!(end_pos, 255);
        assert_eq!(name.len(), 253);
    }

    #[test]
    fn test_name_too_long() {
        // 4 labels of 63 bytes + terminating zero = 257 bytes
        let mut buf = Vec::new();
        for _ in 0..4 {
            buf.push(63);
            buf.extend(std::iter::repeat(b'a').take(63));
        }
        buf.push(0);

        let result = parse_qname(&buf, 0);

        assert_eq!(
            result.unwrap_err(),
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME
        );
    }
}
