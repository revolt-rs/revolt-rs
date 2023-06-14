/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// CollectionStats : Collection stats



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CollectionStats {
    /// Namespace
    #[serde(rename = "ns")]
    pub ns: String,
    /// ISO8601 formatted timestamp
    #[serde(rename = "localTime")]
    pub local_time: String,
    /// Latency stats
    #[serde(rename = "latencyStats")]
    pub latency_stats: ::std::collections::HashMap<String, crate::models::LatencyStats>,
    #[serde(rename = "queryExecStats")]
    pub query_exec_stats: Box<crate::models::CollectionStatsQueryExecStats>,
    /// Number of documents in collection
    #[serde(rename = "count")]
    pub count: i32,
}

impl CollectionStats {
    /// Collection stats
    pub fn new(ns: String, local_time: String, latency_stats: ::std::collections::HashMap<String, crate::models::LatencyStats>, query_exec_stats: crate::models::CollectionStatsQueryExecStats, count: i32) -> CollectionStats {
        CollectionStats {
            ns,
            local_time,
            latency_stats,
            query_exec_stats: Box::new(query_exec_stats),
            count,
        }
    }
}

