//! The Topic-Definition Module contains the [`Topic`],
//! [`ContentFilteredTopic`], and [`MultiTopic`] classes, the TopicListener
//! interface, and more generally, all that is needed by the application to
//! define [`Topic`] objects and attach QoS policies to them.

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct TopicDescription {
    type_name: String,
    name: String,
}

/// A [`Topic`] is identified by its name, which must be unique in the whole
/// Domain. In addition (by virtue of extending [`TopicDescription`]) it fully
/// specifies the type of the data that can be communicated when publishing or
/// subscribing to the [Topic].
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Topic;

/// [`ContentFilteredTopic`] describes a more sophisticated subscription that
/// indicates the subscriber does not want to necessarily see all values of each
/// instance published under the [`Topic`]. Rather, it wants to see only the
/// values whose contents satisfy certain criteria. This class therefore can be
/// used to request content-based subscriptions.
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct ContentFilteredTopic {
    filter_expression: String,
}

/// [`MultiTopic`] allows a more sophisticated subscription that can select and
/// combine data received from multiple [`Topic`]s into a single resulting type
/// (specified by the inherited type_name). The data will then be filtered
/// (selection) and possibly re-arranged (aggregation/projection) according to a
/// subscription_expression with parameters expression_parameters.
#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct MultiTopic {
    subscription_expression: String,
}
