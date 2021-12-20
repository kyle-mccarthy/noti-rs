use std::fmt::{Debug, Display};

use uuid::{fmt::Simple, Uuid};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TemplateId([u8; Simple::LENGTH]);

impl TemplateId {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_str(&self) -> &str {
        // SAFETY: a template is created from a uuid formatted as a simple string.
        // Encoding a uuid as a simple string, guarantees the output is valid
        // utf8.
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }

    pub fn from_uuid(value: Uuid) -> Self {
        let mut buf = [0; Simple::LENGTH];

        let simple = value.simple();
        simple.encode_lower(&mut buf);

        Self(buf)
    }
}

impl Default for TemplateId {
    fn default() -> Self {
        Self::from_uuid(Uuid::new_v4())
    }
}

impl Display for TemplateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for TemplateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TemplateId").field(&self.0).finish()
    }
}

#[cfg(test)]
mod test {
    use super::TemplateId;

    #[test]
    fn test_template_id_as_str() {
        let uuid = uuid::Uuid::new_v4();
        let simple = uuid.simple().to_string();

        let id = TemplateId::from_uuid(uuid);

        assert_eq!(&simple, id.as_str());
    }
}
