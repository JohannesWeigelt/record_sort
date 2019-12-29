///Marker-Trait for everything that can be processed by record_sort.
/// Processing mean
///     - sorting
///     - generating
///     - read from file
///     - write to file
pub trait Record: PartialOrd {}