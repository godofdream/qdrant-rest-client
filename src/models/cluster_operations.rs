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
pub struct ClusterOperations {
    #[serde(rename = "move_shard")]
    pub move_shard: Box<crate::models::MoveShard>,
    #[serde(rename = "replicate_shard")]
    pub replicate_shard: Box<crate::models::MoveShard>,
    #[serde(rename = "abort_transfer")]
    pub abort_transfer: Box<crate::models::MoveShard>,
    #[serde(rename = "drop_replica")]
    pub drop_replica: Box<crate::models::Replica>,
    #[serde(rename = "create_sharding_key")]
    pub create_sharding_key: Box<crate::models::CreateShardingKey>,
    #[serde(rename = "drop_sharding_key")]
    pub drop_sharding_key: Box<crate::models::DropShardingKey>,
}

impl ClusterOperations {
    pub fn new(move_shard: crate::models::MoveShard, replicate_shard: crate::models::MoveShard, abort_transfer: crate::models::MoveShard, drop_replica: crate::models::Replica, create_sharding_key: crate::models::CreateShardingKey, drop_sharding_key: crate::models::DropShardingKey) -> ClusterOperations {
        ClusterOperations {
            move_shard: Box::new(move_shard),
            replicate_shard: Box::new(replicate_shard),
            abort_transfer: Box::new(abort_transfer),
            drop_replica: Box::new(drop_replica),
            create_sharding_key: Box::new(create_sharding_key),
            drop_sharding_key: Box::new(drop_sharding_key),
        }
    }
}


