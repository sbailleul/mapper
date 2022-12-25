use super::aggregated_to::AggregatedTo;

#[derive(Debug, Clone)]
pub struct Attrs<T> {
    pub to: AggregatedTo<T>,
}
