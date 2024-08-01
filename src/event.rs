use downcaster::AsAny;

pub trait Event: AsAny + Sync + Send + 'static {
    fn event_name(&self) -> &'static str;

    fn event_version(&self) -> &'static str {
        "1.0"
    }
}

pub trait DomainEvent: Event {}