#[cfg(test)]
mod tests {
    use crate::exceptions::SCloudException;
    use strum::IntoEnumIterator;

    #[test]
    fn test_exceptions_to_str() {
        let ex_msg_array: [&'static str; 65] = [
            // HEADER SECTION
            "[HEADER_SECTION] Buffer length is less than header length.",
            "[HEADER_SECTION] The header is empty.",
            // QUESTION SECTION
            "[QUESTION_SECTION] Buffer length is less than question section length.",
            "[QUESTION_SECTION] `q_name` too long.",
            // ANSWER SECTION
            "[ANSWER_SECTION] Buffer length is less than answer section length.",
            "[ANSWER_SECTION] Label too long for DNS.",
            "[ANSWER_SECTION] Failed to parse DNS answer section: not enough bytes for TYPE, CLASS, TTL, and RDLENGTH.",
            "[ANSWER_SECTION] Failed to parse DNS answer section: RDLENGTH exceeds remaining buffer size.",
            // AUTHORITY SECTION
            "[AUTHORITY_SECTION] Buffer length is less than authority section length.",
            "[AUTHORITY_SECTION] Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)",
            // ADDITIONAL SECTION
            "[ADDITIONAL_SECTION] Buffer length is less than additional section length.",
            "[ADDITIONAL_SECTION] Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)",
            "[ADDITIONAL_SECTION] `q_name` too long.",
            "[ADDITIONAL_SECTION] Buffer length is less than authority section length.",
            // QNAME
            "[QNAME] Impossible to parse the `q_name`, check if a `q_name` is provided.",
            "[QNAME] Impossible to parse the `q_name`, pos is greater than buffer length.",
            "[QNAME] Impossible to parse the `q_name`, pos and len are greater than buffer length.",
            "[QNAME] Impossible to parse the `q_name`, compression 0xC0xx failed.",
            // QTYPE
            "[QTYPE] Unknown `q_type`.",
            // QCLASS
            "[QCLASS] Unknown `q_class`.",
            "[QCLASS] Unknown `q_class`.",
            // STUB RESOLVER
            "[STUB_RESOLVER] Invalid DNS ID (difference between `response.header.id` and `request_id`).",
            "[STUB_RESOLVER] Invalid DNS response.",
            "[STUB_RESOLVER] Failed to create UDP socket.",
            "[STUB_RESOLVER] Failed to read, socket timeout.",
            "[STUB_RESOLVER] Failed to send to socket.",
            "[STUB_RESOLVER] Failed to receive from socket.",
            // RESOLVER
            "[RESOLVER] DNS response failed validation against the original query.",
            "[RESOLVER] Record out of zone.",
            // ZONES
            "[ZONE_PARSER] Zone file not found.",
            "[ZONE_PARSER] Zone file is empty.",
            "[ZONE_PARSER] `zone_parser()` failed to read the zone file.",
            "[ZONE_PARSER] `zone_parser()` detect TTL field but failed to read this field.",
            // CONFIG
            "[SCLOUD_CONFIG] Configuration file not found.",
            "[SCLOUD_CONFIG] Error while parsing the JSON file.",
            "[SCLOUD_CONFIG] Missing forwarder.",
            "[SCLOUD_CONFIG] Missing address.",
            "[SCLOUD_CONFIG] Error while parsing the IP address.",
            "[SCLOUD_CONFIG] Invalid server port (must be between 1 and 65535).",
            "[SCLOUD_CONFIG] Invalid max UDP payload size.",
            "[SCLOUD_CONFIG] Invalid DNS limits (label length, domain length, or packet size).",
            "[SCLOUD_CONFIG] Invalid listener configuration.",
            "[SCLOUD_CONFIG] Duplicate listener name detected.",
            "[SCLOUD_CONFIG] Invalid listener port.",
            "[SCLOUD_CONFIG] Listener has no valid protocol defined.",
            "[SCLOUD_CONFIG] TLS enabled but certificate path is missing.",
            "[SCLOUD_CONFIG] TLS enabled but private key path is missing.",
            "[SCLOUD_CONFIG] TLS listeners require TCP support.",
            "[SCLOUD_CONFIG] Invalid DNS-over-HTTPS (DoH) configuration.",
            "[SCLOUD_CONFIG] Unknown or invalid ACL reference.",
            "[SCLOUD_CONFIG] Invalid forwarder configuration.",
            "[SCLOUD_CONFIG] Duplicate forwarder name detected.",
            "[SCLOUD_CONFIG] Invalid DNS zone configuration.",
            "[SCLOUD_CONFIG] Duplicate zone name detected.",
            "[SCLOUD_CONFIG] Zone file path is missing.",
            "[SCLOUD_CONFIG] Slave zone has no master servers defined.",
            "[SCLOUD_CONFIG] Forward zone has no forwarders defined.",
            "[SCLOUD_CONFIG] Inline zone is invalid (missing records or SOA).",
            "[SCLOUD_CONFIG] Referenced TSIG key does not exist.",
            "[SCLOUD_CONFIG] MX record is missing priority field.",
            "[SCLOUD_CONFIG] Priority field is only allowed on MX records.",
            "[SCLOUD_CONFIG] Invalid DNS view configuration.",
            "[SCLOUD_CONFIG] Duplicate view name detected.",
            "[SCLOUD_CONFIG] Invalid dynamic update configuration.",
            "[SCLOUD_CONFIG] Dynamic update references an unknown zone.",
        ];

        let mut i = 0;
        for ex in SCloudException::iter() {
            let msg = ex.to_str();
            println!("variant: {:?}, msg: {}", ex, msg);
            assert_eq!(msg, ex_msg_array[i], "Mismatch for variant {:?}", ex);
            i += 1;
        }

        assert_eq!(i, ex_msg_array.len(), "Number of exceptions mismatch");
    }

    #[test]
    fn test_exceptions_debug() {
        for ex in SCloudException::iter() {
            let debug_str = format!("{:?}", ex);
            assert!(!debug_str.is_empty(), "Debug string should not be empty");
        }
    }

    #[test]
    fn test_exceptions_partial_eq() {
        for ex in SCloudException::iter() {
            assert_eq!(ex, ex.clone());
        }
    }

    #[test]
    fn test_exceptions_iter_count() {
        let count = SCloudException::iter().count();
        let expected_count = 65;
        assert_eq!(count, expected_count);
    }

    #[test]
    fn test_all_exceptions_covered() {
        for ex in SCloudException::iter() {
            let msg = ex.to_str();
            assert!(
                !msg.is_empty(),
                "to_str() returned empty string for variant {:?}",
                ex
            );
        }
    }
}
