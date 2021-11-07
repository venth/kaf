use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use shaku::Interface;

use crate::domain::model::{ApplicationProperties, Command, Count, EstimatedQueryRange, K4QError, QueryRange, Topic, TopicsMatcherType};
use crate::domain::model::Progress;
use crate::domain::model::Record;
use crate::domain::model::TopicName;

pub trait RecordFinder: Interface {
    fn find_by<'a>(&self, topic_name: &'a TopicName) -> Pin<Box<dyn Stream<Item=Record>>>;
}

pub trait CommandRecognizer: Interface {
    fn recognize(&self, args: &Vec<&str>) -> Option<Command>;
}

pub trait ProgressNotifier: Interface {
    fn notify(&self, message: &str);
    fn start(&self, estimated_max_size: &Count) -> Arc<dyn Progress>;
}

pub trait TopicsFinder: Interface {
    fn find_by<'a>(&self, topics_matcher_type: &'a TopicsMatcherType) -> Pin<Box<dyn Stream<Item=Topic> + 'a>>;
}

pub trait QueryRangeEstimator: Interface {
    fn estimate(&self, topic: &Topic, query_range: &QueryRange) -> EstimatedQueryRange;
}

pub trait ConfiguredContextFactory: Interface {
    fn create(&self, properties: &dyn ApplicationProperties) -> Box<dyn ConfiguredContext>;
}

pub trait ConfiguredContext {
    fn topics_finder(&self) -> Box<dyn TopicsFinder>;
    fn query_range_estimator(&self) -> Box<dyn QueryRangeEstimator>;
    fn record_finder(&self) -> Box<dyn RecordFinder>;
}

pub trait PropertiesSource: Interface {
    fn load(&self, config_location: &Path) -> Result<Box<dyn ApplicationProperties>, K4QError>;
}
