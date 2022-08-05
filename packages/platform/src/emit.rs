use cosmwasm_std::{Event, Response, Timestamp};
use finance::coin::{Amount, Coin};
use finance::currency::Currency;

use crate::batch::Batch;

pub trait Emit where Self: Sized {
    fn emit<K, V>(self, event_key: K, event_value: V) -> Self
        where
            K: Into<String>,
            V: Into<String>;

    /// Specialization of [`emit`](Batch::emit) for timestamps.
    fn emit_timestamp<K>(self, event_key: K, timestamp: &Timestamp) -> Self
        where
            K: Into<String>,
    {
        self.emit(event_key, timestamp.nanos().to_string())
    }

    /// Specialization of [`emit`](Batch::emit) for values implementing [`ToString`].
    fn emit_to_string_value<K, V>(self, event_key: K, value: V) -> Self
        where
            K: Into<String>,
            V: ToString,
    {
        self.emit(event_key, value.to_string())
    }

    /// Specialization of [`emit`](Batch::emit) for [`Coin`]'s amount.
    fn emit_coin_amount<K, C>(self, event_key: K, coin: Coin<C>) -> Self
        where
            K: Into<String>,
            C: Currency,
    {
        self.emit(event_key, Amount::from(coin).to_string())
    }
}

pub struct Emitter {
    batch: Batch,
    event: Event,
}

impl Emitter {
    pub(crate) fn new<T>(batch: Batch, event_type: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            batch,
            event: Event::new(event_type.into()),
        }
    }
}

impl From<Emitter> for Response
where
    Batch: Into<Response>,
{
    fn from(emitter: Emitter) -> Self {
        Response::from(emitter.batch).add_event(emitter.event)
    }
}

impl Emit for Emitter {
    fn emit<K, V>(mut self, event_key: K, event_value: V) -> Self where K: Into<String>, V: Into<String> {
        self.event = self.event.add_attribute(event_key, event_value);

        self
    }
}