use serde::{de::DeserializeOwned, Serialize};

pub trait Id:
    std::fmt::Debug
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + std::hash::Hash
    + Serialize
    + DeserializeOwned
{
}

impl<T> Id for T where
    T: std::fmt::Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + std::hash::Hash
        + Serialize
        + DeserializeOwned
{
}

#[cfg(test)]
mod test_size {
    #[test]
    fn test_size_of_id() {
        println!("{}", std::mem::size_of::<uuid::Uuid>());
    }
}

// impl From<&'static str> for Id {
//     fn from(id: &'static str) -> Self {
//         Self::Str(id)
//     }
// }

// impl From<String> for Id {
//     fn from(id: String) -> Self {
//         Self::Str(Cow::Owned(id))
//     }
// }
