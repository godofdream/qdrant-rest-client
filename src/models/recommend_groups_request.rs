/*
 * Qdrant API
 *
 * API description for Qdrant vector search engine.  This document describes CRUD and search operations on collections of points (vectors with payload).  Qdrant supports any combinations of `should`, `must` and `must_not` conditions, which makes it possible to use in applications when object could not be described solely by vector. It could be location features, availability flags, and other custom properties businesses should take into account. ## Examples This examples cover the most basic use-cases - collection creation and basic vector search. ### Create collection First - let's create a collection with dot-production metric. ``` curl -X PUT 'http://localhost:6333/collections/test_collection' \\   -H 'Content-Type: application/json' \\   --data-raw '{     \"vectors\": {       \"size\": 4,       \"distance\": \"Dot\"     }   }'  ``` Expected response: ``` {     \"result\": true,     \"status\": \"ok\",     \"time\": 0.031095451 } ``` We can ensure that collection was created: ``` curl 'http://localhost:6333/collections/test_collection' ``` Expected response: ``` {   \"result\": {     \"status\": \"green\",     \"vectors_count\": 0,     \"segments_count\": 5,     \"disk_data_size\": 0,     \"ram_data_size\": 0,     \"config\": {       \"params\": {         \"vectors\": {           \"size\": 4,           \"distance\": \"Dot\"         }       },       \"hnsw_config\": {         \"m\": 16,         \"ef_construct\": 100,         \"full_scan_threshold\": 10000       },       \"optimizer_config\": {         \"deleted_threshold\": 0.2,         \"vacuum_min_vector_number\": 1000,         \"max_segment_number\": 5,         \"memmap_threshold\": 50000,         \"indexing_threshold\": 20000,         \"flush_interval_sec\": 1       },       \"wal_config\": {         \"wal_capacity_mb\": 32,         \"wal_segments_ahead\": 0       }     }   },   \"status\": \"ok\",   \"time\": 2.1199e-05 } ```  ### Add points Let's now add vectors with some payload: ``` curl -L -X PUT 'http://localhost:6333/collections/test_collection/points?wait=true' \\ -H 'Content-Type: application/json' \\ --data-raw '{   \"points\": [     {\"id\": 1, \"vector\": [0.05, 0.61, 0.76, 0.74], \"payload\": {\"city\": \"Berlin\"}},     {\"id\": 2, \"vector\": [0.19, 0.81, 0.75, 0.11], \"payload\": {\"city\": [\"Berlin\", \"London\"] }},     {\"id\": 3, \"vector\": [0.36, 0.55, 0.47, 0.94], \"payload\": {\"city\": [\"Berlin\", \"Moscow\"] }},     {\"id\": 4, \"vector\": [0.18, 0.01, 0.85, 0.80], \"payload\": {\"city\": [\"London\", \"Moscow\"] }},     {\"id\": 5, \"vector\": [0.24, 0.18, 0.22, 0.44], \"payload\": {\"count\": [0]}},     {\"id\": 6, \"vector\": [0.35, 0.08, 0.11, 0.44]}   ] }' ``` Expected response: ``` {     \"result\": {         \"operation_id\": 0,         \"status\": \"completed\"     },     \"status\": \"ok\",     \"time\": 0.000206061 } ``` ### Search with filtering Let's start with a basic request: ``` curl -L -X POST 'http://localhost:6333/collections/test_collection/points/search' \\ -H 'Content-Type: application/json' \\ --data-raw '{     \"vector\": [0.2,0.1,0.9,0.7],     \"top\": 3 }' ``` Expected response: ``` {     \"result\": [         { \"id\": 4, \"score\": 1.362, \"payload\": null, \"version\": 0 },         { \"id\": 1, \"score\": 1.273, \"payload\": null, \"version\": 0 },         { \"id\": 3, \"score\": 1.208, \"payload\": null, \"version\": 0 }     ],     \"status\": \"ok\",     \"time\": 0.000055785 } ``` But result is different if we add a filter: ``` curl -L -X POST 'http://localhost:6333/collections/test_collection/points/search' \\ -H 'Content-Type: application/json' \\ --data-raw '{     \"filter\": {         \"should\": [             {                 \"key\": \"city\",                 \"match\": {                     \"value\": \"London\"                 }             }         ]     },     \"vector\": [0.2, 0.1, 0.9, 0.7],     \"top\": 3 }' ``` Expected response: ``` {     \"result\": [         { \"id\": 4, \"score\": 1.362, \"payload\": null, \"version\": 0 },         { \"id\": 2, \"score\": 0.871, \"payload\": null, \"version\": 0 }     ],     \"status\": \"ok\",     \"time\": 0.000093972 } ``` 
 *
 * The version of the OpenAPI document: v1.7.x
 * Contact: andrey@vasnetsov.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendGroupsRequest {
    #[serde(rename = "shard_key", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shard_key: Option<Option<Box<crate::models::ShardKeySelector>>>,
    /// Look for vectors closest to those
    #[serde(rename = "positive", skip_serializing_if = "Option::is_none")]
    pub positive: Option<Vec<crate::models::RecommendExample>>,
    /// Try to avoid vectors like this
    #[serde(rename = "negative", skip_serializing_if = "Option::is_none")]
    pub negative: Option<Vec<crate::models::RecommendExample>>,
    #[serde(rename = "strategy", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Option<crate::models::RecommendStrategy>>,
    #[serde(rename = "filter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Option<Box<crate::models::Filter>>>,
    #[serde(rename = "params", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub params: Option<Option<Box<crate::models::SearchParams>>>,
    #[serde(rename = "with_payload", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub with_payload: Option<Option<Box<crate::models::WithPayloadInterface>>>,
    #[serde(rename = "with_vector", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub with_vector: Option<Option<Box<crate::models::WithVector>>>,
    /// Define a minimal score threshold for the result. If defined, less similar results will not be returned. Score of the returned result might be higher or smaller than the threshold depending on the Distance function used. E.g. for cosine similarity only higher scores will be returned.
    #[serde(rename = "score_threshold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<Option<f32>>,
    #[serde(rename = "using", skip_serializing_if = "Option::is_none")]
    pub using: Option<String>,
    #[serde(rename = "lookup_from", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lookup_from: Option<Option<Box<crate::models::LookupLocation>>>,
    /// Payload field to group by, must be a string or number field. If the field contains more than 1 value, all values will be used for grouping. One point can be in multiple groups.
    #[serde(rename = "group_by")]
    pub group_by: String,
    /// Maximum amount of points to return per group
    #[serde(rename = "group_size")]
    pub group_size: i32,
    /// Maximum amount of groups to return
    #[serde(rename = "limit")]
    pub limit: i32,
    #[serde(rename = "with_lookup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub with_lookup: Option<Option<Box<crate::models::WithLookupInterface>>>,
}

impl RecommendGroupsRequest {
    pub fn new(group_by: String, group_size: i32, limit: i32) -> RecommendGroupsRequest {
        RecommendGroupsRequest {
            shard_key: None,
            positive: None,
            negative: None,
            strategy: None,
            filter: None,
            params: None,
            with_payload: None,
            with_vector: None,
            score_threshold: None,
            using: None,
            lookup_from: None,
            group_by,
            group_size,
            limit,
            with_lookup: None,
        }
    }
}


