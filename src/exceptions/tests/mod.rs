#[cfg(test)]
mod tests {
    use crate::exceptions::SCloudException;
    use strum::IntoEnumIterator;

    #[test]
    fn test_exceptions_to_str() {
        let ex_msg_array: [&'static str; 74] = [
            // HEADER SECTION
            "Buffer length is less than header length.",
            "The header is empty.",
            // QUESTION SECTION
            "Buffer length is less than question section length.",
            "`q_name` too long.",
            // ANSWER SECTION
            "Buffer length is less than answer section length.",
            "Label too long for DNS.",
            "Failed to parse DNS answer section: not enough bytes for TYPE, CLASS, TTL, and RDLENGTH.",
            "Failed to parse DNS answer section: RDLENGTH exceeds remaining buffer size.",
            // AUTHORITY SECTION
            "Buffer length is less than authority section length.",
            "Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)",
            // ADDITIONAL SECTION
            "Buffer length is less than additional section length.",
            "Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)",
            "`q_name` too long.",
            "Buffer length is less than authority section length.",
            // QNAME
            "Impossible to parse the `q_name`, check if a `q_name` is provided.",
            "Impossible to parse the `q_name`, pos is greater than buffer length.",
            "Impossible to parse the `q_name`, pos and len are greater than buffer length.",
            "Impossible to parse the `q_name`, compression 0xC0xx failed.",
            // QTYPE
            "Unknown `q_type`.",
            // QCLASS
            "Unknown `q_class`.",
            "Unknown `q_class`.",
            // STUB RESOLVER
            "Invalid DNS ID (difference between `response.header.id` and `request_id`).",
            "Invalid DNS response.",
            "Failed to create UDP socket.",
            "Failed to read, socket timeout.",
            "Failed to send to socket.",
            "Failed to receive from socket.",
            // RESOLVER
            "DNS response failed validation against the original query.",
            "Record out of zone.",
            // ZONES
            "Zone file not found.",
            "Zone file is empty.",
            "`zone_parser()` failed to read the zone file.",
            "`zone_parser()` detect TTL field but failed to read this field.",
            // CONFIG
            "Configuration file not found.",
            "Error while parsing the JSON file.",
            "Missing forwarder.",
            "Missing address.",
            "Error while parsing the IP address.",
            "Invalid server port (must be between 1 and 65535).",
            "Invalid max UDP payload size.",
            "Invalid DNS limits (label length, domain length, or packet size).",
            "Invalid listener configuration.",
            "Duplicate listener name detected.",
            "Invalid listener port.",
            "Listener has no valid protocol defined.",
            "TLS enabled but certificate path is missing.",
            "TLS enabled but private key path is missing.",
            "TLS listeners require TCP support.",
            "Invalid DNS-over-HTTPS (DoH) configuration.",
            "Unknown or invalid ACL reference.",
            "Invalid forwarder configuration.",
            "Duplicate forwarder name detected.",
            "Invalid DNS zone configuration.",
            "Duplicate zone name detected.",
            "Zone file path is missing.",
            "Slave zone has no master servers defined.",
            "Forward zone has no forwarders defined.",
            "Inline zone is invalid (missing records or SOA).",
            "Referenced TSIG key does not exist.",
            "MX record is missing priority field.",
            "Priority field is only allowed on MX records.",
            "Invalid DNS view configuration.",
            "Duplicate view name detected.",
            "Invalid dynamic update configuration.",
            "Dynamic update references an unknown zone.",
            // LOGGING
            "Logging path creation failed.",
            "Log file creation/opening failed.",
            // WORKER
            "Failed to link the worker to the thread, and cannot spawn a worker.",
            "`dns_tx` is not set for this worker.",
            "`dns_rx` is not set for this worker.",
            "Listener bind just failed at 'threads::run(&self)'.",
            "Failed to create a decoding worker.",
            "Unknown worker type.",
            // LISTENER
            "Listener revc() failed",
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
        let expected_count = 74;
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
