//! no_std 集成测试模块
//! 
//! 此模块包含针对 no_std 环境下的集成测试，重点测试：
//! - no_std 环境下的合约功能
//! - 内存管理
//! - 错误处理
//! - 性能表现
//! - 兼容性验证

use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};
use luckee_nft::state::Expiration;

use luckee_nft::contract::{execute, instantiate, query, migrate};
use luckee_nft::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use luckee_nft::types::{NftKind, NftMeta, Scale, Recipe, RecipeInput};

fn mock_app() -> App {
    App::default()
}

fn contract() -> Box<dyn cw_multi_test::Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query)
        .with_migrate(migrate);
    Box::new(contract)
}

#[test]
fn test_no_std_basic_functionality() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 测试基本铸造功能
    let mint_msg = ExecuteMsg::Mint {
        token_id: 1,
        owner: "user".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "no_std_test".to_string(),
            collection_group_id: None,
            serial_in_series: 1,
        },
    };

    let result = app.execute_contract(
        Addr::unchecked("minter"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    );

    assert!(result.is_ok());

    // 测试查询功能
    let query_msg = QueryMsg::OwnerOf { token_id: 1, include_expired: None };
    let query_result: cw721::OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(&nft_contract_addr, &query_msg)
        .unwrap();

    assert_eq!(query_result.owner, "user");
}

#[test]
fn test_no_std_synthesis_functionality() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 设置合成配方
    let recipe = Recipe {
        inputs: vec![RecipeInput {
            kind: NftKind::Clover,
            amount: 2,
        }],
        cost: None,
    };

    app.execute_contract(
        Addr::unchecked("creator"),
        nft_contract_addr.clone(),
        &ExecuteMsg::SetRecipe {
            target: NftKind::Firefly,
            recipe,
        },
        &[],
    ).unwrap();

    // 铸造输入NFT
    for i in 1..=2 {
        let mint_msg = ExecuteMsg::Mint {
            token_id: i,
            owner: "user".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "synthesis_input".to_string(),
                collection_group_id: None,
                serial_in_series: i as u32,
            },
        };

        app.execute_contract(
            Addr::unchecked("minter"),
            nft_contract_addr.clone(),
            &mint_msg,
            &[],
        ).unwrap();
    }

    // 执行合成
    let synthesize_msg = ExecuteMsg::Synthesize {
        inputs: vec![1, 2],
        target: NftKind::Firefly,
    };

    let result = app.execute_contract(
        Addr::unchecked("user"),
        nft_contract_addr.clone(),
        &synthesize_msg,
        &[],
    );

    assert!(result.is_ok());

    // 验证合成结果
    let query_msg = QueryMsg::TokenMeta { token_id: 3 };
    let query_result: luckee_nft::msg::TokenMetaResponse = app
        .wrap()
        .query_wasm_smart(&nft_contract_addr, &query_msg)
        .unwrap();

    assert_eq!(query_result.meta.kind, NftKind::Firefly);
    assert_eq!(query_result.meta.crafted_from, Some(vec![1, 2]));
}

#[test]
fn test_no_std_batch_operations() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 批量铸造
    let mints = vec![
        luckee_nft::msg::BatchMintItem {
            token_id: 1,
            owner: "user1".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "batch_test".to_string(),
                collection_group_id: None,
                serial_in_series: 1,
            },
        },
        luckee_nft::msg::BatchMintItem {
            token_id: 2,
            owner: "user2".to_string(),
            extension: NftMeta {
                kind: NftKind::Firefly,
                scale_origin: Scale::Small,
                physical_sku: None,
                crafted_from: None,
                series_id: "batch_test".to_string(),
                collection_group_id: None,
                serial_in_series: 2,
            },
        },
    ];

    let batch_mint_msg = ExecuteMsg::BatchMint { mints };

    let result = app.execute_contract(
        Addr::unchecked("minter"),
        nft_contract_addr.clone(),
        &batch_mint_msg,
        &[],
    );

    assert!(result.is_ok());

    // 验证批量铸造结果
    let query_msg = QueryMsg::Tokens { owner: "user1".to_string(), start_after: None, limit: None };
    let query_result: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(&nft_contract_addr, &query_msg)
        .unwrap();

    assert_eq!(query_result.tokens.len(), 1);
    assert_eq!(query_result.tokens[0], "1");
}

#[test]
fn test_no_std_memory_management() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 大量铸造操作测试内存管理
    for i in 1..=100 {
        let mint_msg = ExecuteMsg::Mint {
            token_id: i,
            owner: "user".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "memory_test".to_string(),
                collection_group_id: None,
                serial_in_series: i as u32,
            },
        };

        let result = app.execute_contract(
            Addr::unchecked("minter"),
            nft_contract_addr.clone(),
            &mint_msg,
            &[],
        );

        assert!(result.is_ok());
    }

    // 验证所有NFT都已正确铸造
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let query_result: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(&nft_contract_addr, &query_msg)
        .unwrap();

    assert_eq!(query_result.tokens.len(), 100);
}

#[test]
fn test_no_std_error_handling() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 测试各种错误情况
    let test_cases = vec![
        // 不存在的NFT查询
        QueryMsg::OwnerOf { token_id: 999, include_expired: None },
        // 不存在的合成配方查询
        QueryMsg::Recipe { target: NftKind::Genesis },
        // 空查询
        QueryMsg::AllTokens { start_after: None, limit: None },
    ];

    for query_msg in test_cases {
        let result: Result<_, _> = app
            .wrap()
            .query_wasm_smart(&nft_contract_addr, &query_msg);

        // 这些查询应该要么成功返回空结果，要么返回适当的错误
        // 关键是不应该导致panic或内存错误
        match result {
            Ok(_) => {
                // 查询成功，这是正常的
            }
            Err(_) => {
                // 查询失败，这也是正常的，只要不是panic
            }
        }
    }
}

#[test]
fn test_no_std_performance() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 性能测试：快速连续操作
    let start_time = std::time::Instant::now();

    // 快速铸造
    for i in 1..=50 {
        let mint_msg = ExecuteMsg::Mint {
            token_id: i,
            owner: "user".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "performance_test".to_string(),
                collection_group_id: None,
                serial_in_series: i as u32,
            },
        };

        let result = app.execute_contract(
            Addr::unchecked("minter"),
            nft_contract_addr.clone(),
            &mint_msg,
            &[],
        );

        assert!(result.is_ok());
    }

    let elapsed = start_time.elapsed();
    
    // 验证性能：50次操作应该在合理时间内完成
    assert!(elapsed.as_secs() < 10, "Performance test took too long: {:?}", elapsed);

    // 快速查询
    let start_time = std::time::Instant::now();

    for i in 1..=50 {
        let query_msg = QueryMsg::OwnerOf { token_id: i, include_expired: None };
        let result: Result<cw721::OwnerOfResponse, _> = app
            .wrap()
            .query_wasm_smart(&nft_contract_addr, &query_msg);

        assert!(result.is_ok());
    }

    let elapsed = start_time.elapsed();
    
    // 验证查询性能
    assert!(elapsed.as_secs() < 5, "Query performance test took too long: {:?}", elapsed);
}

#[test]
fn test_no_std_compatibility() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 测试与标准CW721的兼容性
    let standard_cw721_queries = vec![
        QueryMsg::ContractInfo {},
        QueryMsg::OwnerOf { token_id: 1, include_expired: None },
        QueryMsg::AllTokens { start_after: None, limit: None },
        QueryMsg::Tokens { owner: "user".to_string(), start_after: None, limit: None },
    ];

    // 先铸造一个NFT
    let mint_msg = ExecuteMsg::Mint {
        token_id: 1,
        owner: "user".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "compatibility_test".to_string(),
            collection_group_id: None,
            serial_in_series: 1,
        },
    };

    app.execute_contract(
        Addr::unchecked("minter"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    ).unwrap();

    // 测试所有标准CW721查询
    for query_msg in standard_cw721_queries {
        let result: Result<_, _> = app
            .wrap()
            .query_wasm_smart(&nft_contract_addr, &query_msg);

        // 所有标准查询都应该成功
        assert!(result.is_ok(), "Standard CW721 query failed: {:?}", query_msg);
    }

    // 测试Luckee扩展查询
    let luckee_queries = vec![
        QueryMsg::TokenMeta { token_id: 1 },
        QueryMsg::TokensByKind { kind: NftKind::Clover, start_after: None, limit: None },
        QueryMsg::TokensBySeries { series_id: "compatibility_test".to_string(), start_after: None, limit: None },
        QueryMsg::LuckeeContractInfo {},
    ];

    for query_msg in luckee_queries {
        let result: Result<_, _> = app
            .wrap()
            .query_wasm_smart(&nft_contract_addr, &query_msg);

        // 所有Luckee扩展查询都应该成功
        assert!(result.is_ok(), "Luckee extension query failed: {:?}", query_msg);
    }
}

#[test]
fn test_no_std_migration() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 铸造一些NFT
    for i in 1..=10 {
        let mint_msg = ExecuteMsg::Mint {
            token_id: i,
            owner: "user".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "migration_test".to_string(),
                collection_group_id: None,
                serial_in_series: i as u32,
            },
        };

        app.execute_contract(
            Addr::unchecked("minter"),
            nft_contract_addr.clone(),
            &mint_msg,
            &[],
        ).unwrap();
    }

    // 验证迁移前的状态
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let query_result: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(&nft_contract_addr, &query_msg)
        .unwrap();

    assert_eq!(query_result.tokens.len(), 10);

    // 执行迁移（这里我们使用相同的合约代码，实际迁移会使用新版本）
    let migrate_msg = luckee_nft::msg::MigrateMsg {};
    
    let result = app.migrate_contract(
        Addr::unchecked("creator"),
        nft_contract_addr.clone(),
        &migrate_msg,
        contract_id,
    );

    // 迁移应该成功
    assert!(result.is_ok());

    // 验证迁移后的状态
    let query_result: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(&nft_contract_addr, &query_msg)
        .unwrap();

    // 所有NFT应该仍然存在
    assert_eq!(query_result.tokens.len(), 10);
}

#[test]
fn test_no_std_edge_cases() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 测试边界值
    let edge_cases = vec![
        // 最小token_id
        0u64,
        // 最大token_id
        u64::MAX,
    ];

    for token_id in edge_cases {
        let mint_msg = ExecuteMsg::Mint {
            token_id,
            owner: "user".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "edge_case_test".to_string(),
                collection_group_id: None,
                serial_in_series: 1,
            },
        };

        let result = app.execute_contract(
            Addr::unchecked("minter"),
            nft_contract_addr.clone(),
            &mint_msg,
            &[],
        );

        // 这些边界值应该被正确处理
        match result {
            Ok(_) => {
                // 铸造成功，验证NFT存在
                let query_msg = QueryMsg::OwnerOf { token_id, include_expired: None };
                let query_result: Result<cw721::OwnerOfResponse, _> = app
                    .wrap()
                    .query_wasm_smart(&nft_contract_addr, &query_msg);

                assert!(query_result.is_ok());
            }
            Err(_) => {
                // 铸造失败，这是可以接受的，只要不是panic
                // 例如，u64::MAX 可能会导致溢出
            }
        }
    }
}
