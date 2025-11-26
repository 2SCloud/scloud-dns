use crate::dns::records::{DNSClass, DNSRecordType};
use crate::exceptions::SCloudException;


#[derive(Debug)]
pub struct QuestionSection {
    pub q_name: String,
    pub q_type: DNSRecordType,
    pub q_class: DNSClass,
}

impl QuestionSection {
    /// Serialize the DNS question section into a byte array
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.q_name.len() + 5);

        for label in self.q_name.split('.') {
            let len = label.len();
            if len > 63 {
                panic!("Label too long for DNS: {}", label);
            }
            buf.push(len.try_into().expect("label length fits in u8"));
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0x00);

        let q_type_u16 = u16::try_from(self.q_type)
            .expect("Cannot convert QuestionSection q_type to u16");
        let q_class_u16 = u16::from(self.q_class);

        buf.extend_from_slice(&q_type_u16.to_be_bytes());
        buf.extend_from_slice(&q_class_u16.to_be_bytes());

        buf
    }


    /// Deserialize the DNS question section from a byte array
    pub fn from_bytes(buf: &[u8]) -> Result<(QuestionSection, usize), SCloudException> {

        let (q_name_bytes, consumed_name) = until_null(buf)?;
        let q_name = String::from_utf8(q_name_bytes.to_vec())
            .map_err(|_| SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED)?;
        let mut pos = consumed_name + 1;

        if buf.len() < pos + 4 {
            return Err(SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED);
        }

        let q_type_bytes = [buf[pos], buf[pos + 1]];
        let q_class_bytes = [buf[pos + 2], buf[pos + 3]];
        pos += 4;

        let q_type = DNSRecordType::try_from(u16::from_be_bytes(q_type_bytes))?;
        let q_class = DNSClass::from(u16::from_be_bytes(q_class_bytes));

        Ok((
            QuestionSection {
                q_name,
                q_type,
                q_class,
            },
            pos
        ))
    }


}

fn until_null(buf: &[u8]) -> Result<(&[u8], usize), SCloudException> {
    match buf.iter().position(|&b| b == 0x00) {
        Some(pos) => Ok((&buf[..pos], buf[..pos].len())),
        None => Err(SCloudException::SCLOUD_QUESTION_IMPOSSIBLE_PARSE_QNAME),
    }
}