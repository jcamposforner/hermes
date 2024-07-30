use downcaster::AsAny;

pub trait Event: AsAny + 'static {
    fn event_name(&self) -> &'static str;
}
