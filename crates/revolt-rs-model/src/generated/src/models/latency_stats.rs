/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// LatencyStats : Collection latency stats



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LatencyStats {
    /// Total operations
    #[serde(rename = "ops")]
    pub ops: i64,
    /// Timestamp at which data keeping begun
    #[serde(rename = "latency")]
    pub latency: i64,
    /// Histogram representation of latency data
    #[serde(rename = "histogram")]
    pub histogram: Vec<crate::models::LatencyHistogramEntry>,
}

impl LatencyStats {
    /// Collection latency stats
    pub fn new(ops: i64, latency: i64, histogram: Vec<crate::models::LatencyHistogramEntry>) -> LatencyStats {
        LatencyStats {
            ops,
            latency,
            histogram,
        }
    }
}

