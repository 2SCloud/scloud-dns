use crate::dns::packet::additional::AdditionalSection;
use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::authority::AuthoritySection;
use crate::dns::packet::header::Header;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;
use crate::utils::ErrorCondition;

pub mod header;
pub(crate) mod question;
mod answer;
mod additional;
mod authority;

pub struct DNSPacket {
    pub header: Header,
    pub questions: Vec<QuestionSection>,
    pub answers: Vec<AnswerSection>,
    pub authorities: Vec<AuthoritySection>,
    pub additionals: Vec<AdditionalSection>,
}

impl DNSPacket {
    const DNS_HEADER_LEN: usize = 12;

    pub fn from_bytes(buf: &[u8]) -> Result<DNSPacket, SCloudException> {
        let mut pos = 0;

        let header = Header::from_bytes(&buf[pos..])?;
        pos += Header::DNS_HEADER_LEN;

        let (question_section, consumed_q) =
            QuestionSection::from_bytes(&buf[pos..])?;
        pos += consumed_q;

        let (answer_section, consumed_a) =
            AnswerSection::from_bytes(&buf[pos..])?;
        pos += consumed_a;

        let (authority_section, consumed_ns) =
            AuthoritySection::from_bytes(&buf[pos..])?;
        pos += consumed_ns;

        let (additional_section, consumed_add) =
            AdditionalSection::from_bytes(&buf[pos..])?;
        pos += consumed_add;


        Ok(DNSPacket {
            header,
            question_section,
            answer_section,
            authority_section,
            additional_section
        })
    }
}
