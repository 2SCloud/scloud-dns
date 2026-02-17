use strum_macros::EnumIter;
use crate::dns::q_class::DNSClass;

mod tests;

#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, EnumIter, Clone)]
pub enum SCloudException {
    // HEADER SECTION
    SCLOUD_HEADER_DESERIALIZATION_FAILED = 0,
    SCLOUD_HEADER_BYTES_EMPTY = 1,

    // QUESTION SECTION
    SCLOUD_QUESTION_DESERIALIZATION_FAILED = 2,
    SCLOUD_QUESTION_SERIALIZATION_FAILED_QNAME_TOO_LONG = 3,

    // ANSWER SECTION
    SCLOUD_ANSWER_DESERIALIZATION_FAILED = 4,
    SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG = 5,
    SCLOUD_IMPOSSIBLE_PARSE_ANSWER_HEADER_TOO_SHORT = 6,
    SCLOUD_IMPOSSIBLE_PARSE_ANSWER_RDATA_OUT_OF_BOUNDS = 7,

    // AUTHORITY SECTION
    SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS = 8,
    SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT = 9,

    // ADDITIONAL SECTION
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED = 10,
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_BUF_TOO_SHORT = 11,
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_QNAME_TOO_LONG = 12,
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS = 13,

    // QNAME
    SCLOUD_IMPOSSIBLE_PARSE_QNAME = 14,
    SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF = 15,
    SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF = 16,
    SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED = 17,

    // QTYPE
    SCLOUD_QTYPE_UNKNOWN_TYPE = 18,

    //QCLASS
    SCLOUD_QCLASS_U16_FOR_DNSCLASS_UNKNOWN = 19,
    SCLOUD_QCLASS_DNSCLASS_FOR_U16_UNKNOWN = 20,

    // STUB RESOLVER
    SCLOUD_STUB_RESOLVER_INVALID_DNS_ID = 21,
    SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE = 22,
    SCLOUD_STUB_RESOLVER_FAILED_TO_CREATE_SOCKET = 23,
    SCLOUD_STUB_RESOLVER_FAILED_TO_READ_SOCKET_TIMEOUT = 24,
    SCLOUD_STUB_RESOLVER_FAILED_TO_SEND_TO_SOCKET = 25,
    SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET = 26,

    // RESOLVER
    SCLOUD_RESOLVER_RESPONSE_MISMATCH = 27,
    SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE = 28,
    SCLOUD_RESOLVER_ANSWER_QNAME_MISMATCH = 29,
    SCLOUD_RESOLVER_AUTHORITY_QNAME_MISMATCH = 30,
    SCLOUD_RESOLVER_ADDITIONNAL_QNAME_MISMATCH = 31,

    // ZONES
    SCLOUD_ZONE_PARSER_FILE_NOT_FOUND = 32,
    SCLOUD_ZONE_PARSER_FILE_EMPTY = 33,
    SCLOUD_ZONE_PARSER_FAILED_TO_READ_ZONE_FILE = 34,
    SCLOUD_ZONE_PARSER_FAILED_TO_READ_TTL_FIELD = 35,

    // CONFIG
    SCLOUD_CONFIG_FILE_NOT_FOUND = 36,
    SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_JSON = 37,
    SCLOUD_CONFIG_MISSING_FORWARDER = 38,
    SCLOUD_CONFIG_MISSING_ADDRESS = 39,
    SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR = 40,
    SCLOUD_CONFIG_INVALID_SERVER_PORT = 41,
    SCLOUD_CONFIG_INVALID_MAX_UDP_PAYLOAD = 42,
    SCLOUD_CONFIG_INVALID_DNS_LIMITS = 43,
    SCLOUD_CONFIG_INVALID_LISTENER = 44,
    SCLOUD_CONFIG_DUPLICATE_LISTENER_NAME = 45,
    SCLOUD_CONFIG_INVALID_LISTENER_PORT = 46,
    SCLOUD_CONFIG_INVALID_LISTENER_PROTOCOLS = 47,
    SCLOUD_CONFIG_TLS_MISSING_CERT = 48,
    SCLOUD_CONFIG_TLS_MISSING_KEY = 49,
    SCLOUD_CONFIG_TLS_REQUIRES_TCP = 50,
    SCLOUD_CONFIG_INVALID_DOH = 51,
    SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE = 52,
    SCLOUD_CONFIG_INVALID_FORWARDER = 53,
    SCLOUD_CONFIG_DUPLICATE_FORWARDER_NAME = 54,
    SCLOUD_CONFIG_INVALID_ZONE = 55,
    SCLOUD_CONFIG_DUPLICATE_ZONE_NAME = 56,
    SCLOUD_CONFIG_ZONE_MISSING_FILE = 57,
    SCLOUD_CONFIG_SLAVE_MISSING_MASTERS = 58,
    SCLOUD_CONFIG_FORWARD_ZONE_MISSING_FORWARDERS = 59,
    SCLOUD_CONFIG_INVALID_INLINE_ZONE = 60,
    SCLOUD_CONFIG_UNKNOWN_TSIG_KEY = 61,
    SCLOUD_CONFIG_MX_MISSING_PRIORITY = 62,
    SCLOUD_CONFIG_PRIORITY_ON_NON_MX = 63,
    SCLOUD_CONFIG_INVALID_VIEW = 64,
    SCLOUD_CONFIG_DUPLICATE_VIEW_NAME = 65,
    SCLOUD_CONFIG_INVALID_DYNUPDATE = 66,
    SCLOUD_CONFIG_DYNUPDATE_UNKNOWN_ZONE = 67,

    // LOGGING
    SCLOUD_LOGGING_PATH_CREATION_FAILED = 68,
    SCLOUD_LOGGING_FILE_CREATION_OR_OPENING_FAILED = 69,

    // WORKER
    SCLOUD_WORKER_FAILED_TO_SPAWN = 70,
    SCLOUD_WORKER_TX_NOT_SET = 71,
    SCLOUD_WORKER_RX_NOT_SET = 72,
    SCLOUD_WORKER_LISTENER_BIND_FAILED = 73,
    SCLOUD_WORKER_FAILED_TO_CREATE_DECODER = 74,
    SCLOUD_WORKER_UNKNOWN_TYPE = 75,

    // LISTENER
    SCLOUD_WORKER_LISTENER_RECV_FAILED = 76,

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

impl TryFrom<u16> for SCloudException {
    type Error = SCloudException;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(SCloudException::SCLOUD_HEADER_DESERIALIZATION_FAILED),
            1 => Ok(SCloudException::SCLOUD_HEADER_BYTES_EMPTY),
            2 => Ok(SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED),
            3 => Ok(SCloudException::SCLOUD_QUESTION_SERIALIZATION_FAILED_QNAME_TOO_LONG),
            4 => Ok(SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED),
            5 => Ok(SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG),
            6 => Ok(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_HEADER_TOO_SHORT),
            7 => Ok(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_RDATA_OUT_OF_BOUNDS),
            8 => Ok(SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS),
            9 => Ok(SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT),
            10 => Ok(SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED),
            11 => Ok(SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_BUF_TOO_SHORT),
            12 => Ok(SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_QNAME_TOO_LONG),
            13 => Ok(SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS),
            14 => Ok(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME),
            15 => Ok(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF),
            16 => Ok(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF),
            17 => Ok(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED),
            18 => Ok(SCloudException::SCLOUD_QTYPE_UNKNOWN_TYPE),
            19 => Ok(SCloudException::SCLOUD_QCLASS_U16_FOR_DNSCLASS_UNKNOWN),
            20 => Ok(SCloudException::SCLOUD_QCLASS_DNSCLASS_FOR_U16_UNKNOWN),
            21 => Ok(SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_ID),
            22 => Ok(SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE),
            23 => Ok(SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_CREATE_SOCKET),
            24 => Ok(SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_READ_SOCKET_TIMEOUT),
            25 => Ok(SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_SEND_TO_SOCKET),
            26 => Ok(SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET),
            27 => Ok(SCloudException::SCLOUD_RESOLVER_RESPONSE_MISMATCH),
            28 => Ok(SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE),
            29 => Ok(SCloudException::SCLOUD_RESOLVER_ANSWER_QNAME_MISMATCH),
            30 => Ok(SCloudException::SCLOUD_RESOLVER_AUTHORITY_QNAME_MISMATCH),
            31 => Ok(SCloudException::SCLOUD_RESOLVER_ADDITIONNAL_QNAME_MISMATCH),
            32 => Ok(SCloudException::SCLOUD_ZONE_PARSER_FILE_NOT_FOUND),
            33 => Ok(SCloudException::SCLOUD_ZONE_PARSER_FILE_EMPTY),
            34 => Ok(SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_ZONE_FILE),
            35 => Ok(SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_TTL_FIELD),
            36 => Ok(SCloudException::SCLOUD_CONFIG_FILE_NOT_FOUND),
            37 => Ok(SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_JSON),
            38 => Ok(SCloudException::SCLOUD_CONFIG_MISSING_FORWARDER),
            39 => Ok(SCloudException::SCLOUD_CONFIG_MISSING_ADDRESS),
            40 => Ok(SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR),
            41 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_SERVER_PORT),
            42 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_MAX_UDP_PAYLOAD),
            43 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_DNS_LIMITS),
            44 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_LISTENER),
            45 => Ok(SCloudException::SCLOUD_CONFIG_DUPLICATE_LISTENER_NAME),
            46 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_LISTENER_PORT),
            47 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_LISTENER_PROTOCOLS),
            48 => Ok(SCloudException::SCLOUD_CONFIG_TLS_MISSING_CERT),
            49 => Ok(SCloudException::SCLOUD_CONFIG_TLS_MISSING_KEY),
            50 => Ok(SCloudException::SCLOUD_CONFIG_TLS_REQUIRES_TCP),
            51 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_DOH),
            52 => Ok(SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE),
            53 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_FORWARDER),
            54 => Ok(SCloudException::SCLOUD_CONFIG_DUPLICATE_FORWARDER_NAME),
            55 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_ZONE),
            56 => Ok(SCloudException::SCLOUD_CONFIG_DUPLICATE_ZONE_NAME),
            57 => Ok(SCloudException::SCLOUD_CONFIG_ZONE_MISSING_FILE),
            58 => Ok(SCloudException::SCLOUD_CONFIG_SLAVE_MISSING_MASTERS),
            59 => Ok(SCloudException::SCLOUD_CONFIG_FORWARD_ZONE_MISSING_FORWARDERS),
            60 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_INLINE_ZONE),
            61 => Ok(SCloudException::SCLOUD_CONFIG_UNKNOWN_TSIG_KEY),
            62 => Ok(SCloudException::SCLOUD_CONFIG_MX_MISSING_PRIORITY),
            63 => Ok(SCloudException::SCLOUD_CONFIG_PRIORITY_ON_NON_MX),
            64 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_VIEW),
            65 => Ok(SCloudException::SCLOUD_CONFIG_DUPLICATE_VIEW_NAME),
            66 => Ok(SCloudException::SCLOUD_CONFIG_INVALID_DYNUPDATE),
            67 => Ok(SCloudException::SCLOUD_CONFIG_DYNUPDATE_UNKNOWN_ZONE),
            68 => Ok(SCloudException::SCLOUD_LOGGING_PATH_CREATION_FAILED),
            69 => Ok(SCloudException::SCLOUD_LOGGING_FILE_CREATION_OR_OPENING_FAILED),
            70 => Ok(SCloudException::SCLOUD_WORKER_FAILED_TO_SPAWN),
            71 => Ok(SCloudException::SCLOUD_WORKER_TX_NOT_SET),
            72 => Ok(SCloudException::SCLOUD_WORKER_RX_NOT_SET),
            73 => Ok(SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED),
            74 => Ok(SCloudException::SCLOUD_WORKER_FAILED_TO_CREATE_DECODER),
            75 => Ok(SCloudException::SCLOUD_WORKER_UNKNOWN_TYPE),
            76 => Ok(SCloudException::SCLOUD_WORKER_LISTENER_RECV_FAILED),

            _ => Err(SCloudException::SCLOUD_WORKER_UNKNOWN_TYPE),
        }
    }
}

impl TryFrom<SCloudException> for u16 {
    type Error = SCloudException;

    fn try_from(c: DNSClass) -> Result<u16, Self::Error> {
        #[allow(unreachable_patterns)]
        match c {
            DNSClass::NONE => Ok(0),
            DNSClass::IN => Ok(1),
            DNSClass::CS => Ok(2),
            DNSClass::CH => Ok(3),
            DNSClass::HS => Ok(4),
            DNSClass::ANY => Ok(255),
            _ => Err(SCloudException::SCLOUD_QCLASS_DNSCLASS_FOR_U16_UNKNOWN),
        }
    }
}