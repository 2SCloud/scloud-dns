mod header;
mod question;
mod answer;
mod authority;

#[cfg(test)]
mod tests {
    use crate::dns::packet::DNSPacket;
    use crate::dns::packet::header::Header;
    use crate::dns::packet::question::QuestionSection;
    use crate::dns::packet::answer::AnswerSection;
    use crate::dns::packet::authority::AuthoritySection;
    use crate::dns::records::{DNSClass, DNSRecordType};

    #[test]
    fn test_dns_packet_from_bytes() {

        let bytes: &[u8] = &[
            // ===== DNS HEADER =====
            0xAA, 0xAA, // ID
            0x01, 0x00, // Flags: standard query response, no error
            0x00, 0x01, // QDCOUNT = 1
            0x00, 0x01, // ANCOUNT = 1
            0x00, 0x01, // NSCOUNT = 1
            0x00, 0x00, // ARCOUNT = 0

            // ===== QUESTION SECTION =====
            0x04, b'r', b'u', b's', b't',
            0x06, b't', b'r', b'e', b'n', b'd', b's',
            0x03, b'c', b'o', b'm',
            0x00,             // end of QNAME
            0x00, 0x01,       // QTYPE = A
            0x00, 0x01,       // QCLASS = IN

            // ===== ANSWER SECTION =====
            0x04, b'r', b'u', b's', b't',
            0x06, b't', b'r', b'e', b'n', b'd', b's',
            0x03, b'c', b'o', b'm',
            0x00,             // NAME
            0x00, 0x01,       // TYPE = A
            0x00, 0x01,       // CLASS = IN
            0x00, 0x00, 0x00, 0x3C, // TTL = 60
            0x00, 0x04,       // RDLENGTH = 4
            0x7F, 0x00, 0x00, 0x01, // RDATA = 127.0.0.1

            // ===== AUTHORITY SECTION (NS) =====
            0x04, b'r', b'u', b's', b't',
            0x06, b't', b'r', b'e', b'n', b'd', b's',
            0x03, b'c', b'o', b'm',
            0x00,             // NAME
            0x00, 0x02,       // TYPE = NS
            0x00, 0x01,       // CLASS = IN
            0x00, 0x00, 0x00, 0x3C, // TTL = 60
            0x00, 0x10,       // RDLENGTH = 16
            0x03, b'n', b's', b'1',
            0x06, b't', b'r', b'e', b'n', b'd', b's',
            0x03, b'c', b'o', b'm',
            0x00,             // end of NS name
        ];

        let expected_packet = DNSPacket{
            header: Header {
                id: 43690,
                qr: false,
                opcode: 0,
                aa: false,
                tc: false,
                rd: true,
                ra: false,
                z: 0,
                rcode: 0,
                qdcount: 1,
                ancount: 1,
                nscount: 1,
                arcount: 0,
            },
            questions: vec![QuestionSection{
                q_name: "rust.trends.com".to_string(),
                q_type: DNSRecordType::A,
                q_class: DNSClass::IN
            }],
            answers: vec![AnswerSection {
                q_name: "rust.trends.com".to_string(),
                r_type: DNSRecordType::A,
                r_class: DNSClass::IN,
                ttl: 60,
                rdlength: 4,
                rdata: vec![127, 0, 0, 1],
            }],
            authorities: vec![AuthoritySection{
                q_name: "rust.trends.com".to_string(),
                q_type: DNSRecordType::NS,
                q_class: DNSClass::IN,
                ttl: 60,
                ns_name: "ns1.trends.com".to_string()
            }],
            additionals: vec![],
        };

        let result = DNSPacket::from_bytes(bytes).unwrap();
        println!("expected: {:?}, got: {:?}", expected_packet, result);
        assert_eq!(expected_packet, result)

    }

    #[test]
    fn test_dns_packet_to_bytes() {
        let packet: DNSPacket = DNSPacket{
            header: Header {
                id: 43690,
                qr: false,
                opcode: 0,
                aa: false,
                tc: false,
                rd: true,
                ra: false,
                z: 0,
                rcode: 0,
                qdcount: 1,
                ancount: 1,
                nscount: 0,
                arcount: 0,
            },
            questions: vec![QuestionSection{
                q_name: "rust.trends.com".to_string(),
                q_type: DNSRecordType::A,
                q_class: DNSClass::IN,
            }],
            answers: vec![AnswerSection{
                q_name: "rust.trends.com".to_string(),
                r_type: DNSRecordType::A,
                r_class: DNSClass::IN,
                ttl: 0,
                rdlength: 0,
                rdata: vec![],
            }],
            authorities: vec![],
            additionals: vec![],
        };

        let expected_bytes: &[u8] = &[0xAA,0xAA,
            0x01, 0x00,
            0x00, 0x01,
            0x00, 0x01,
            0x00,
            0x00,
            0x00,
            0x00, 0x04, 0x72, 0x75, 0x73, 0x74,
            0x06, 0x74, 0x72, 0x65, 0x6E, 0x64, 0x73,
            0x03, 0x63, 0x6F, 0x6D,
            0x00,
            0x00, 0x01,
            0x00, 0x01,
            0x04, 0x72, 0x75, 0x73,
            0x74, 0x06, 0x74, 0x72,
            0x65, 0x6E, 0x64, 0x73,
            0x03, 0x63, 0x6F, 0x6D,
            0x00, 0x00, 0x01, 0x00,
            0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00
        ];

        let result = DNSPacket::to_bytes(packet).unwrap();

        println!("expected: {:?}\ngot: {:?}", expected_bytes, result);
        assert_eq!(expected_bytes, result);
    }

}