// External imports
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Range {
    #[serde(rename = "entityId")]
    variation_id: String,
    end_of_range: u64,
}

#[derive(Debug)]
pub struct TrafficAllocation(BTreeMap<u64, String>);

impl TrafficAllocation {
    pub fn variation(&self, bucket_value: u64) -> Option<&str> {
        // Use BTreeMap::range to find the variation in O(log(n))
        self.0
            .range(bucket_value..)
            .next()
            .map(|(_, variation)| variation.as_ref())
    }
}

impl<'de> Deserialize<'de> for TrafficAllocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut tree = BTreeMap::new();
        for range in Vec::<Range>::deserialize(deserializer)? {
            tree.insert(range.end_of_range, range.variation_id);
        }

        Ok(Self(tree))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variation() {
        let traffic_allocation = {
            let mut ranges = BTreeMap::<u64, String>::new();
            ranges.insert(3_333, String::from("A"));
            ranges.insert(6_666, String::from("B"));
            ranges.insert(10_000, String::from("C"));

            TrafficAllocation(ranges)
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
