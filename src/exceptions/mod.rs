use strum_macros::EnumIter;

mod tests;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, EnumIter, Clone)]
pub enum SCloudException {
    // HEADER SECTION
    SCLOUD_HEADER_DESERIALIZATION_FAILED,
    SCLOUD_HEADER_BYTES_EMPTY,

    // QUESTION SECTION
    SCLOUD_QUESTION_DESERIALIZATION_FAILED,
    SCLOUD_QUESTION_SERIALIZATION_FAILED_QNAME_TOO_LONG,

    // ANSWER SECTION
    SCLOUD_ANSWER_DESERIALIZATION_FAILED,
    SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG,
    SCLOUD_IMPOSSIBLE_PARSE_ANSWER_HEADER_TOO_SHORT,
    SCLOUD_IMPOSSIBLE_PARSE_ANSWER_RDATA_OUT_OF_BOUNDS,

    // AUTHORITY SECTION
    SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS,
    SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT,

    // ADDITIONAL SECTION
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED,
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_BUF_TOO_SHORT,
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_QNAME_TOO_LONG,
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS,

    // QNAME
    SCLOUD_IMPOSSIBLE_PARSE_QNAME,
    SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF,
    SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF,
    SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED,

    // QTYPE
    SCLOUD_QTYPE_UNKNOWN_TYPE,

    //QCLASS
    SCLOUD_QCLASS_U16_FOR_DNSCLASS_UNKNOWN,
    SCLOUD_QCLASS_DNSCLASS_FOR_U16_UNKNOWN,

    // STUB RESOLVER
    SCLOUD_STUB_RESOLVER_INVALID_DNS_ID,
    SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE,
    SCLOUD_STUB_RESOLVER_FAILED_TO_CREATE_SOCKET,
    SCLOUD_STUB_RESOLVER_FAILED_TO_READ_SOCKET_TIMEOUT,
    SCLOUD_STUB_RESOLVER_FAILED_TO_SEND_TO_SOCKET,
    SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET,

    // RESOLVER
    SCLOUD_RESOLVER_RESPONSE_MISMATCH,
    SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE,
    SCLOUD_RESOLVER_ANSWER_QNAME_MISMATCH,
    SCLOUD_RESOLVER_AUTHORITY_QNAME_MISMATCH,
    SCLOUD_RESOLVER_ADDITIONNAL_QNAME_MISMATCH,

    // ZONES
    SCLOUD_ZONE_PARSER_FILE_NOT_FOUND,
    SCLOUD_ZONE_PARSER_FILE_EMPTY,
    SCLOUD_ZONE_PARSER_FAILED_TO_READ_ZONE_FILE,
    SCLOUD_ZONE_PARSER_FAILED_TO_READ_TTL_FIELD,

    // CONFIG
    SCLOUD_CONFIG_FILE_NOT_FOUND,
    SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_JSON,
    SCLOUD_CONFIG_MISSING_FORWARDER,
    SCLOUD_CONFIG_MISSING_ADDRESS,
    SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR,
    SCLOUD_CONFIG_INVALID_SERVER_PORT,
    SCLOUD_CONFIG_INVALID_MAX_UDP_PAYLOAD,
    SCLOUD_CONFIG_INVALID_DNS_LIMITS,
    SCLOUD_CONFIG_INVALID_LISTENER,
    SCLOUD_CONFIG_DUPLICATE_LISTENER_NAME,
    SCLOUD_CONFIG_INVALID_LISTENER_PORT,
    SCLOUD_CONFIG_INVALID_LISTENER_PROTOCOLS,
    SCLOUD_CONFIG_TLS_MISSING_CERT,
    SCLOUD_CONFIG_TLS_MISSING_KEY,
    SCLOUD_CONFIG_TLS_REQUIRES_TCP,
    SCLOUD_CONFIG_INVALID_DOH,
    SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE,
    SCLOUD_CONFIG_INVALID_FORWARDER,
    SCLOUD_CONFIG_DUPLICATE_FORWARDER_NAME,
    SCLOUD_CONFIG_INVALID_ZONE,
    SCLOUD_CONFIG_DUPLICATE_ZONE_NAME,
    SCLOUD_CONFIG_ZONE_MISSING_FILE,
    SCLOUD_CONFIG_SLAVE_MISSING_MASTERS,
    SCLOUD_CONFIG_FORWARD_ZONE_MISSING_FORWARDERS,
    SCLOUD_CONFIG_INVALID_INLINE_ZONE,
    SCLOUD_CONFIG_UNKNOWN_TSIG_KEY,
    SCLOUD_CONFIG_MX_MISSING_PRIORITY,
    SCLOUD_CONFIG_PRIORITY_ON_NON_MX,
    SCLOUD_CONFIG_INVALID_VIEW,
    SCLOUD_CONFIG_DUPLICATE_VIEW_NAME,
    SCLOUD_CONFIG_INVALID_DYNUPDATE,
    SCLOUD_CONFIG_DYNUPDATE_UNKNOWN_ZONE,

    // LOGGING
    SCLOUD_LOGGING_PATH_CREATION_FAILED,
    SCLOUD_LOGGING_FILE_CREATION_OR_OPENING_FAILED,

    // WORKER
    SCLOUD_WORKER_FAILED_TO_SPAWN,
    SCLOUD_WORKER_TX_NOT_SET,
    SCLOUD_WORKER_RX_NOT_SET,
    SCLOUD_WORKER_LISTENER_BIND_FAILED,
    SCLOUD_WORKER_FAILED_TO_CREATE_DECODER,
    SCLOUD_WORKER_UNKNOWN_TYPE,

    // LISTENER
    SCLOUD_WORKER_LISTENER_RECV_FAILED,

    // DECODER
}

impl SCloudException {
    #[allow(unused)]
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            //HEADER SECTION
            SCloudException::SCLOUD_HEADER_DESERIALIZATION_FAILED => {
                "Buffer length is less than header length."
            }

            SCloudException::SCLOUD_HEADER_BYTES_EMPTY => "The header is empty.",

            // QUESTION SECTION
            SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED => {
                "Buffer length is less than question section length."
            }
            SCloudException::SCLOUD_QUESTION_SERIALIZATION_FAILED_QNAME_TOO_LONG => {
                "`q_name` too long."
            }

            // ANSWER SECTION
            SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED => {
                "Buffer length is less than answer section length."
            }
            SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG => {
                "Label too long for DNS."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_HEADER_TOO_SHORT => {
                "Failed to parse DNS answer section: not enough bytes for TYPE, CLASS, TTL, and RDLENGTH."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_RDATA_OUT_OF_BOUNDS => {
                "Failed to parse DNS answer section: RDLENGTH exceeds remaining buffer size."
            }

            // AUTHORITY SECTION
            SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS => {
                "Buffer length is less than authority section length."
            }
            SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT => {
                "Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)"
            }

            // ADDITIONAL SECTION
            SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED => {
                "Buffer length is less than additional section length."
            }
            SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_BUF_TOO_SHORT => {
                "Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)"
            }
            SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_QNAME_TOO_LONG => {
                "`q_name` too long."
            }
            SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS => {
                "Buffer length is less than authority section length."
            }

            // QNAME
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME => {
                "Impossible to parse the `q_name`, check if a `q_name` is provided."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF => {
                "Impossible to parse the `q_name`, pos is greater than buffer length."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF => {
                "Impossible to parse the `q_name`, pos and len are greater than buffer length."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED => {
                "Impossible to parse the `q_name`, compression 0xC0xx failed."
            }

            // QTYPE
            SCloudException::SCLOUD_QTYPE_UNKNOWN_TYPE => "Unknown `q_type`.",

            //QCLASS
            SCloudException::SCLOUD_QCLASS_U16_FOR_DNSCLASS_UNKNOWN => {
                "Unknown `q_class`."
            }
            SCloudException::SCLOUD_QCLASS_DNSCLASS_FOR_U16_UNKNOWN => {
                "Unknown `q_class`."
            }

            // STUB RESOLVER
            SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_ID => {
                "Invalid DNS ID (difference between `response.header.id` and `request_id`)."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE => {
                "Invalid DNS response."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_CREATE_SOCKET => {
                "Failed to create UDP socket."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_READ_SOCKET_TIMEOUT => {
                "Failed to read, socket timeout."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_SEND_TO_SOCKET => {
                "Failed to send to socket."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET => {
                "Failed to receive from socket."
            }

            // RESOLVER
            SCloudException::SCLOUD_RESOLVER_RESPONSE_MISMATCH => {
                "DNS response failed validation against the original query."
            }
            SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE => "Record out of zone.",
            SCloudException::SCLOUD_RESOLVER_ANSWER_QNAME_MISMATCH => {
                "`AnswerSection.q_name` is not the same as `QuestionSection.q_name`"
            }
            SCloudException::SCLOUD_RESOLVER_AUTHORITY_QNAME_MISMATCH => {
                "`AuthoritySection.q_name` is not the same as `QuestionSection.q_name`"
            }
            SCloudException::SCLOUD_RESOLVER_ADDITIONNAL_QNAME_MISMATCH => {
                "`AdditionnalSection.q_name` is not the same as `QuestionSection.q_name`"
            }

            // ZONES
            SCloudException::SCLOUD_ZONE_PARSER_FILE_NOT_FOUND => {
                "Zone file not found."
            }
            SCloudException::SCLOUD_ZONE_PARSER_FILE_EMPTY => "Zone file is empty.",
            SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_ZONE_FILE => {
                "`zone_parser()` failed to read the zone file."
            }
            SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_TTL_FIELD => {
                "`zone_parser()` detect TTL field but failed to read this field."
            }

            //CONFIG
            SCloudException::SCLOUD_CONFIG_FILE_NOT_FOUND => {
                "Configuration file not found."
            }
            SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_JSON => {
                "Error while parsing the JSON file."
            }
            SCloudException::SCLOUD_CONFIG_MISSING_FORWARDER => {
                "Missing forwarder."
            }
            SCloudException::SCLOUD_CONFIG_MISSING_ADDRESS => "Missing address.",
            SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR => {
                "Error while parsing the IP address."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_SERVER_PORT => {
                "Invalid server port (must be between 1 and 65535)."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_MAX_UDP_PAYLOAD => {
                "Invalid max UDP payload size."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_DNS_LIMITS => {
                "Invalid DNS limits (label length, domain length, or packet size)."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_LISTENER => {
                "Invalid listener configuration."
            }
            SCloudException::SCLOUD_CONFIG_DUPLICATE_LISTENER_NAME => {
                "Duplicate listener name detected."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_LISTENER_PORT => {
                "Invalid listener port."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_LISTENER_PROTOCOLS => {
                "Listener has no valid protocol defined."
            }
            SCloudException::SCLOUD_CONFIG_TLS_MISSING_CERT => {
                "TLS enabled but certificate path is missing."
            }
            SCloudException::SCLOUD_CONFIG_TLS_MISSING_KEY => {
                "TLS enabled but private key path is missing."
            }
            SCloudException::SCLOUD_CONFIG_TLS_REQUIRES_TCP => {
                "TLS listeners require TCP support."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_DOH => {
                "Invalid DNS-over-HTTPS (DoH) configuration."
            }
            SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE => {
                "Unknown or invalid ACL reference."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_FORWARDER => {
                "Invalid forwarder configuration."
            }
            SCloudException::SCLOUD_CONFIG_DUPLICATE_FORWARDER_NAME => {
                "Duplicate forwarder name detected."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_ZONE => {
                "Invalid DNS zone configuration."
            }
            SCloudException::SCLOUD_CONFIG_DUPLICATE_ZONE_NAME => {
                "Duplicate zone name detected."
            }
            SCloudException::SCLOUD_CONFIG_ZONE_MISSING_FILE => {
                "Zone file path is missing."
            }
            SCloudException::SCLOUD_CONFIG_SLAVE_MISSING_MASTERS => {
                "Slave zone has no master servers defined."
            }
            SCloudException::SCLOUD_CONFIG_FORWARD_ZONE_MISSING_FORWARDERS => {
                "Forward zone has no forwarders defined."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_INLINE_ZONE => {
                "Inline zone is invalid (missing records or SOA)."
            }
            SCloudException::SCLOUD_CONFIG_UNKNOWN_TSIG_KEY => {
                "Referenced TSIG key does not exist."
            }
            SCloudException::SCLOUD_CONFIG_MX_MISSING_PRIORITY => {
                "MX record is missing priority field."
            }
            SCloudException::SCLOUD_CONFIG_PRIORITY_ON_NON_MX => {
                "Priority field is only allowed on MX records."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_VIEW => {
                "Invalid DNS view configuration."
            }
            SCloudException::SCLOUD_CONFIG_DUPLICATE_VIEW_NAME => {
                "Duplicate view name detected."
            }
            SCloudException::SCLOUD_CONFIG_INVALID_DYNUPDATE => {
                "Invalid dynamic update configuration."
            }
            SCloudException::SCLOUD_CONFIG_DYNUPDATE_UNKNOWN_ZONE => {
                "Dynamic update references an unknown zone."
            }

            // LOGGING
            SCloudException::SCLOUD_LOGGING_PATH_CREATION_FAILED => {
                "Logging path creation failed."
            }
            SCloudException::SCLOUD_LOGGING_FILE_CREATION_OR_OPENING_FAILED => {
                "Log file creation/opening failed."
            }

            // WORKER
            SCloudException::SCLOUD_WORKER_FAILED_TO_SPAWN => {
                "Failed to link the worker to the thread, and cannot spawn a worker."
            }
            SCloudException::SCLOUD_WORKER_TX_NOT_SET => {
                "`dns_tx` is not set for this worker."
            }
            SCloudException::SCLOUD_WORKER_RX_NOT_SET => {
                "`dns_rx` is not set for this worker."
            }
            SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED => {
                "Listener bind just failed at 'threads::run(&self)'."
            }
            SCloudException::SCLOUD_WORKER_FAILED_TO_CREATE_DECODER => {
                "Failed to create a decoding worker."
            }
            SCloudException::SCLOUD_WORKER_UNKNOWN_TYPE => {
                "Unknown worker type."
            }

            // LISTENER
            SCloudException::SCLOUD_WORKER_LISTENER_RECV_FAILED => {
                "Listener revc() failed"
            }
            _ => "Unknown error.",
        }
    }
}
