use std::pin::Pin;

use futures::{Stream, StreamExt};
use futures::stream;
use shaku::Component;

use crate::domain::model::{KeyValue, Offset, Payload};
use crate::domain::model::Partition;
use crate::domain::model::Record;
use crate::domain::model::TopicName;
use crate::domain::ports;

impl ports::RecordFinder for KafkaRecordFinder {
    fn find_by<'a>(&self, topic_name: &'a TopicName) -> Pin<Box<dyn Stream<Item=Record>>> {
        let topic_name = topic_name.clone();
        stream::repeat(move ||
            Record::of(
                topic_name,
                KeyValue::from("key"),
                Partition::from(&0),
                Offset::from(&0),
                Payload::from("{}"),
            ))
            .take(4280)
            .map(|f| f())
            .boxed()
    }
}


#[derive(Component)]
#[shaku(interface = ports::RecordFinder)]
pub struct KafkaRecordFinder {}