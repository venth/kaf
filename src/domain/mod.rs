use std::sync::Arc;

use shaku::{HasComponent, module};

pub(crate) mod ports;
pub(crate) mod action;
pub(crate) mod service;
pub(crate) mod query;
pub(crate) mod model;
mod app;
mod prepared_command;


pub trait CliModule: HasComponent<dyn ports::CommandRecognizer> {}

pub trait KafkaModule: HasComponent<dyn ports::RecordFinder> + HasComponent<dyn ports::TopicsFinder> + HasComponent<dyn ports::QueryRangeEstimator> {}

pub trait ConsoleModule: HasComponent<dyn ports::ProgressNotifier> {}

pub trait PropertiesModule: HasComponent<dyn ports::PropertiesSource> {}

pub trait Module: HasComponent<dyn service::App> {}

module! {
    DomainModule: Module {
        components = [app::AppImpl],
        providers = [],

        use CliModule {
            components = [ports::CommandRecognizer],
            providers = [],
        },

        use KafkaModule {
            components = [ports::RecordFinder, ports::TopicsFinder, ports::QueryRangeEstimator],
            providers = [],
        },

        use ConsoleModule {
            components = [ports::ProgressNotifier],
            providers = []
        },

        use PropertiesModule {
            components = [ports::PropertiesSource],
            providers = []
        }
    }
}

pub fn module(cli_module: Arc<dyn CliModule>,
              kafka_module: Arc<dyn KafkaModule>,
              console_module: Arc<dyn ConsoleModule>,
              properties_module: Arc<dyn PropertiesModule>) -> Arc<dyn Module> {
    Arc::new(DomainModule::builder(cli_module, kafka_module, console_module, properties_module)
        .build())
}
