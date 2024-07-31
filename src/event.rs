use downcaster::AsAny;

pub trait Event: AsAny + Sync + Send + 'static {
    fn event_name(&self) -> &'static str;
}

pub trait DomainEvent: Event {}