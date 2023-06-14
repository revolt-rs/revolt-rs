pub struct Id<T: Marker> {
    phantom: core::marker::PhantomData<fn(T) -> T>,
    inner: String,
}

impl<T: Marker> Id<T> {
    #[must_use]
    pub fn get(&self) -> &str {
        &self.inner
    }
}

pub trait Marker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct UserMarker;
impl Marker for UserMarker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct ServerMarker;
impl Marker for ServerMarker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct ChannelMarker;
impl Marker for ChannelMarker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct MessageMarker;
impl Marker for MessageMarker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct ReportMarker;
impl Marker for ReportMarker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct StrikeMarker;
impl Marker for StrikeMarker {}

#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct SessionMarker;
impl Marker for SessionMarker {}
