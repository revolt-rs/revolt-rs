use non_empty_string::NonEmptyString;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Id<T: Marker> {
    phantom: core::marker::PhantomData<T>,
    inner: String,
}

impl<T: Marker> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl<T: Marker> Id<T> {
    const fn from_string(inner: String) -> Self {
        Self {
            phantom: core::marker::PhantomData,
            inner,
        }
    }

    #[track_caller]
    pub fn new(value: &str) -> Self {
        // if let Some(value) = NonEmptyString::new(value.to_string()) {
        //     Some(Self::from_string(value.to_string()))
        // } else {
        //     None
        // } 

        return Self::from_string(String::from(value.to_string()));
    }

    #[must_use]
    pub fn get(&self) -> &str {
        &self.inner
    }
}

pub trait Marker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct GenericMarker;
impl Marker for GenericMarker {}

#[derive(Clone, Copy, Hash, Debug, Default, PartialEq)]
pub struct UserMarker;
impl Marker for UserMarker {}

#[derive(Clone, Copy, Hash, Debug, Default, PartialEq)]
pub struct ServerMarker;
impl Marker for ServerMarker {}

#[derive(Clone, Copy, Hash, Debug, Default, PartialEq)]
pub struct ChannelMarker;
impl Marker for ChannelMarker {}

#[derive(Clone, Copy, Hash, Debug, Default, PartialEq)]
pub struct MessageMarker;
impl Marker for MessageMarker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct ReportMarker;
impl Marker for ReportMarker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct StrikeMarker;
impl Marker for StrikeMarker {}

#[derive(Clone, Copy, Hash, Debug, Default, PartialEq)]
pub struct EmojiMarker;
impl Marker for EmojiMarker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct SessionMarker;
impl Marker for SessionMarker {}

impl<T: Marker> serde::Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.inner)
    }
}

impl<'de, T: Marker> serde::Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(IdVisitor {
            phantom: std::marker::PhantomData,
        })
    }
}

struct IdVisitor<T: Marker> {
    phantom: std::marker::PhantomData<T>,
}

impl<'de, T: Marker> serde::de::Visitor<'de> for IdVisitor<T> {
    type Value = Id<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string with the revolt ID format")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Id {
            phantom: core::marker::PhantomData,
            inner: v.to_string(),
        })
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Id {
            phantom: core::marker::PhantomData,
            inner: v,
        })
    }
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Id {
            phantom: core::marker::PhantomData,
            inner: v.to_string(),
        })
    }
}
