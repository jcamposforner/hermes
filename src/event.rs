use downcaster::AsAny;

pub trait Event: AsAny + Sync + Send + 'static {}

pub trait EventIdentifiable {
    fn event_name() -> &'static str;
}

pub trait DomainEvent: Event {}