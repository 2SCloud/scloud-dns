#[cfg(test)]
mod tests {
    use crate::exceptions::SCloudException;
    use strum::IntoEnumIterator;
    use crate::dns::q_type::DNSRecordType;

    #[test]
    fn test_tryfrom_u16_for_dns_record_type() {
        for rtype in DNSRecordType::iter() {
            if let DNSRecordType::Unknown(_) = rtype {
                continue;
            }

            let code = u16::try_from(rtype).unwrap();
            let back = DNSRecordType::try_from(code).unwrap();

            assert_eq!(back, rtype, "Round-trip failed for {:?}", rtype);
        }
    }

    #[test]
    fn test_tryfrom_u16_for_dns_record_type_failed() {
        let code = u16::try_from(DNSRecordType::Unknown(24616)).err().unwrap();

        assert_eq!(code, SCloudException::SCLOUD_QTYPE_UNKNOWN_TYPE);
    }

    #[test]
    fn test_tryfrom_dns_record_type_for_u16() {
        let result = DNSRecordType::try_from(39).unwrap();
        println!("expected: {:?}\ngot: {:?}", DNSRecordType::DNAME, result);
        assert_eq!(result, DNSRecordType::DNAME);
    }

    #[test]
    fn test_tryfrom_dns_record_type_for_u16_failed() {
        let result = DNSRecordType::try_from(45616).err().unwrap();
        println!(
            "expected: {:?}\ngot: {:?}",
            SCloudException::SCLOUD_QTYPE_UNKNOWN_TYPE,
            result
        );
        assert_eq!(result, SCloudException::SCLOUD_QTYPE_UNKNOWN_TYPE);
    }

    #[test]
    fn test_tryfrom_array_for_dns_record_type() {
        let bytes: &[u8; 2] = &[0x00, 0x27];
        let result = DNSRecordType::try_from(bytes).unwrap();
        println!("expected: {:?}\ngot: {:?}", DNSRecordType::DNAME, result);
        assert_eq!(result, DNSRecordType::DNAME);
    }

    #[test]
    fn test_tryfrom_array_for_dns_record_type_failed() {
        let bytes: &[u8; 2] = &[0x01, 0x80];
        let result = DNSRecordType::try_from(bytes).err().unwrap();
        println!(
            "expected: {:?}\ngot: {:?}",
            SCloudException::SCLOUD_QTYPE_UNKNOWN_TYPE,
            result
        );
        assert_eq!(result, SCloudException::SCLOUD_QTYPE_UNKNOWN_TYPE);
    }
}
