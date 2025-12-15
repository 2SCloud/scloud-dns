#[cfg(test)]
mod tests {
    use crate::dns::packet::authority::AuthoritySection;
    use crate::exceptions::SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT;
    use crate::exceptions::SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS;

    #[test]
    fn test_authority_buf_too_short() {
        let buf = vec![0x00];
        let result = AuthoritySection::from_bytes(&buf, 0);

        println!(
            "expected: SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_LOWER_THAN_POS10\ngot: {:?}",
            AuthoritySection::from_bytes(&buf, 0).err().unwrap()
        );

        assert_eq!(
            result.err().unwrap(),
            SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT
        );
    }

    #[test]
    fn test_authority_rdata_out_of_bounds() {
        let mut buf: Vec<u8> = vec![
            0x03, b'f', b'o', b'o', 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x3C, 0x00,
            0x10, 0x03, b'n', b's', b'1',
        ];

        let result = AuthoritySection::from_bytes(&buf, 0);
        assert!(matches!(
            result,
            Err(SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS)
        ));
    }
}
