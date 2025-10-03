//! 边界测试模块
//! 
//! 此模块包含针对合约边界条件的测试，重点测试：
//! - 合成边界测试：超出 MAX_SYNTHESIS_INPUTS 时的错误路径
//! - 批量铸造边界与重复 token_id 测试
//! - 权限测试：非 minter 调用 mint 应被拒绝
//! - pause/unpause 行为测试（在 pause 下禁止操作）
//! - 数值溢出测试
//! - 输入验证测试

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
fn test_synthesis_input_limit() {
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
            amount: 1,
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

    // 铸造大量NFT用于测试
    let mut token_ids = Vec::new();
    for i in 1..=60 {
        let mint_msg = ExecuteMsg::Mint {
            token_id: i,
            owner: "user".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "test".to_string(),
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

        token_ids.push(i);
    }

    // 测试超出 MAX_SYNTHESIS_INPUTS (50) 的合成操作
    let too_many_inputs = token_ids[..51].to_vec(); // 51个输入，超出限制
    
    let synthesize_msg = ExecuteMsg::Synthesize {
        inputs: too_many_inputs,
        target: NftKind::Firefly,
    };

    let result = app.execute_contract(
        Addr::unchecked("user"),
        nft_contract_addr.clone(),
        &synthesize_msg,
        &[],
    );

    // 应该失败，因为输入数量超出限制
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Too many inputs"));
}

#[test]
fn test_batch_mint_limit() {
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

    // 创建超出 MAX_BATCH_MINT (100) 的批量铸造请求
    let mut mints = Vec::new();
    for i in 1..=101 {
        mints.push(luckee_nft::msg::BatchMintItem {
            token_id: i,
            owner: "user".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "batch_test".to_string(),
                collection_group_id: None,
                serial_in_series: i as u32,
            },
        });
    }

    let batch_mint_msg = ExecuteMsg::BatchMint { mints };

    let result = app.execute_contract(
        Addr::unchecked("minter"),
        nft_contract_addr.clone(),
        &batch_mint_msg,
        &[],
    );

    // 应该失败，因为批量数量超出限制
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Too many tokens"));
}

#[test]
fn test_duplicate_token_id_in_batch() {
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

    // 创建包含重复 token_id 的批量铸造请求
    let mints = vec![
        luckee_nft::msg::BatchMintItem {
            token_id: 1,
            owner: "user1".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "duplicate_test".to_string(),
                collection_group_id: None,
                serial_in_series: 1,
            },
        },
        luckee_nft::msg::BatchMintItem {
            token_id: 1, // 重复的 token_id
            owner: "user2".to_string(),
            extension: NftMeta {
                kind: NftKind::Firefly,
                scale_origin: Scale::Small,
                physical_sku: None,
                crafted_from: None,
                series_id: "duplicate_test".to_string(),
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

    // 应该失败，因为存在重复的 token_id
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Token already exists"));
}

#[test]
fn test_unauthorized_minter() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "authorized_minter".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 非授权地址尝试铸造
    let mint_msg = ExecuteMsg::Mint {
        token_id: 1,
        owner: "user".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "unauthorized_test".to_string(),
            collection_group_id: None,
            serial_in_series: 1,
        },
    };

    let result = app.execute_contract(
        Addr::unchecked("unauthorized_minter"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    );

    // 应该失败，因为铸造者未授权
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Minter not authorized"));
}

#[test]
fn test_pause_unpause_behavior() {
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

    // 暂停合约
    let pause_msg = ExecuteMsg::Pause {};
    app.execute_contract(
        Addr::unchecked("creator"),
        nft_contract_addr.clone(),
        &pause_msg,
        &[],
    ).unwrap();

    // 尝试在暂停状态下铸造
    let mint_msg = ExecuteMsg::Mint {
        token_id: 1,
        owner: "user".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "pause_test".to_string(),
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

    // 应该失败，因为合约已暂停
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Contract is paused"));

    // 恢复合约
    let unpause_msg = ExecuteMsg::Unpause {};
    app.execute_contract(
        Addr::unchecked("creator"),
        nft_contract_addr.clone(),
        &unpause_msg,
        &[],
    ).unwrap();

    // 现在应该可以正常铸造
    let mint_result = app.execute_contract(
        Addr::unchecked("minter"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    );

    // 应该成功
    assert!(mint_result.is_ok());
}

#[test]
fn test_overflow_protection() {
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

    // 测试最大 u64 值的 token_id
    let max_token_id = u64::MAX;
    let mint_msg = ExecuteMsg::Mint {
        token_id: max_token_id,
        owner: "user".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "overflow_test".to_string(),
            collection_group_id: None,
            serial_in_series: 1,
        },
    };

    // 这应该成功，因为 u64::MAX 是有效的 token_id
    let result = app.execute_contract(
        Addr::unchecked("minter"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    );

    assert!(result.is_ok());

    // 尝试铸造下一个 token_id，这会导致溢出
    let next_mint_msg = ExecuteMsg::Mint {
        token_id: max_token_id + 1, // 这会导致溢出
        owner: "user".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "overflow_test".to_string(),
            collection_group_id: None,
            serial_in_series: 2,
        },
    };

    // 由于 Rust 的溢出检查，这应该失败
    let result = app.execute_contract(
        Addr::unchecked("minter"),
        nft_contract_addr.clone(),
        &next_mint_msg,
        &[],
    );

    // 应该失败，因为 token_id 溢出
    assert!(result.is_err());
}

#[test]
fn test_invalid_address_format() {
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

    // 使用无效地址格式尝试铸造
    let mint_msg = ExecuteMsg::Mint {
        token_id: 1,
        owner: "invalid_address".to_string(), // 无效地址格式
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "invalid_address_test".to_string(),
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

    // 应该失败，因为地址格式无效
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid address"));
}

#[test]
fn test_empty_inputs_synthesis() {
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

    // 尝试使用空输入进行合成
    let synthesize_msg = ExecuteMsg::Synthesize {
        inputs: vec![], // 空输入
        target: NftKind::Firefly,
    };

    let result = app.execute_contract(
        Addr::unchecked("user"),
        nft_contract_addr.clone(),
        &synthesize_msg,
        &[],
    );

    // 应该失败，因为输入为空
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Recipe not found") || 
            result.unwrap_err().to_string().contains("Insufficient input tokens"));
}

#[test]
fn test_nonexistent_token_operations() {
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

    // 尝试转移不存在的 NFT
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: "user2".to_string(),
        token_id: 999, // 不存在的 token_id
    };

    let result = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &transfer_msg,
        &[],
    );

    // 应该失败，因为 NFT 不存在
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Token not found"));

    // 尝试销毁不存在的 NFT
    let burn_msg = ExecuteMsg::Burn { token_id: 999 };

    let result = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &burn_msg,
        &[],
    );

    // 应该失败，因为 NFT 不存在
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Token not found"));
}

#[test]
fn test_unauthorized_admin_operations() {
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

    // 非管理员尝试设置合成配方
    let recipe = Recipe {
        inputs: vec![RecipeInput {
            kind: NftKind::Clover,
            amount: 2,
        }],
        cost: None,
    };

    let set_recipe_msg = ExecuteMsg::SetRecipe {
        target: NftKind::Firefly,
        recipe,
    };

    let result = app.execute_contract(
        Addr::unchecked("unauthorized_user"),
        nft_contract_addr.clone(),
        &set_recipe_msg,
        &[],
    );

    // 应该失败，因为用户未授权
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unauthorized"));

    // 非管理员尝试暂停合约
    let pause_msg = ExecuteMsg::Pause {};

    let result = app.execute_contract(
        Addr::unchecked("unauthorized_user"),
        nft_contract_addr.clone(),
        &pause_msg,
        &[],
    );

    // 应该失败，因为用户未授权
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unauthorized"));
}

#[test]
fn test_series_id_validation() {
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

    // 尝试使用空系列ID铸造
    let mint_msg = ExecuteMsg::Mint {
        token_id: 1,
        owner: "user".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "".to_string(), // 空系列ID
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

    // 应该失败，因为系列ID为空
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid series ID"));
}

#[test]
fn test_collection_group_id_validation() {
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

    // 尝试使用无效集合组ID铸造
    let mint_msg = ExecuteMsg::Mint {
        token_id: 1,
        owner: "user".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "test_series".to_string(),
            collection_group_id: Some("".to_string()), // 空集合组ID
            serial_in_series: 1,
        },
    };

    let result = app.execute_contract(
        Addr::unchecked("minter"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    );

    // 应该失败，因为集合组ID为空
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid collection group ID"));
}
