// External imports
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
struct Range {
    #[serde(rename = "entityId")]
    variation_id: String,
    #[serde(rename = "endOfRange")]
    end: u64,
}

#[derive(Debug)]
pub struct TrafficAllocation(BTreeMap<u64, String>);

impl TrafficAllocation {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<TrafficAllocation, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut tree = BTreeMap::new();
        for range in Vec::<Range>::deserialize(deserializer)? {
            tree.insert(range.end, range.variation_id);
        }
        Ok(TrafficAllocation(tree))
    }

    #[allow(dead_code)]
    pub fn variation(&self, bucket_value: u64) -> Option<&str> {
        // Use BTreeMap::range to find the variation in O(log(n))
        self.0
            .range(bucket_value..)
            .next()
            .map(|(_, variation)| variation.as_ref())
    }
}

/// Macro to create TrafficAllocation
/// Currently only used for testing
#[cfg(test)]
macro_rules! traffic_allocation {
    { $( $end_of_range: literal => $variation: expr),* $(,)?} => {
        {
            let mut ranges = BTreeMap::<u64, String>::new();

            $(
                ranges.insert($end_of_range, $variation);
            )*

            TrafficAllocation(ranges)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variation() {
        let traffic_allocation = traffic_allocation! {
            3_333 => String::from("A"),
            6_666 => String::from("B"),
            10_000 => String::from("C"),
        };

        assert_eq!(traffic_allocation.variation(0), Some("A"));
        assert_eq!(traffic_allocation.variation(1_000), Some("A"));
        assert_eq!(traffic_allocation.variation(2_000), Some("A"));
        assert_eq!(traffic_allocation.variation(3_000), Some("A"));
        assert_eq!(traffic_allocation.variation(4_000), Some("B"));
        assert_eq!(traffic_allocation.variation(5_000), Some("B"));
        assert_eq!(traffic_allocation.variation(6_000), Some("B"));
        assert_eq!(traffic_allocation.variation(7_000), Some("C"));
        assert_eq!(traffic_allocation.variation(8_000), Some("C"));
        assert_eq!(traffic_allocation.variation(9_000), Some("C"));
        assert_eq!(traffic_allocation.variation(10_000), Some("C"));
        assert_eq!(traffic_allocation.variation(11_000), None);
        assert_eq!(traffic_allocation.variation(99_000), None);
    }
}
