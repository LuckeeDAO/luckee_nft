use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};
use luckee_nft::msg::{ExecuteMsg, InstantiateMsg};
use luckee_nft::types::{NftKind, NftMeta, Scale};

// 导入合约代码
use luckee_nft::contract::{execute, instantiate, query, migrate};

fn mock_app() -> App {
    App::default()
}

fn contract() -> Box<dyn cw_multi_test::Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query)
        .with_migrate(migrate);
    Box::new(contract)
}

#[test]
fn test_enforce_external_config() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    // 部署NFT合约
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "blind_box_contract".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 测试默认配置（允许metadata-only模式）
    let mint_msg = ExecuteMsg::Mint {
        token_id: 1,
        owner: "user1".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "series_1".to_string(),
            collection_group_id: None,
            serial_in_series: 1,
        },
    };

    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    );
    if res.is_err() {
        println!("First mint error: {:?}", res.as_ref().err());
    }
    assert!(res.is_ok());

    // UpdateEnforceExternal 已被移除，本地 CW721 模式下 mint 应该成功
    let mint_msg = ExecuteMsg::Mint {
        token_id: 2,
        owner: "user1".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "series_2".to_string(),
            collection_group_id: None,
            serial_in_series: 1,
        },
    };

    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    );
    if res.is_err() {
        println!("Error: {:?}", res.as_ref().err());
    }
    assert!(res.is_ok());

    // SetNftContract 已被移除，本地 CW721 模式下 mint 应该成功

    // 本地 CW721 模式下 mint 应该成功（使用不同的 token_id）
    let mint_msg2 = ExecuteMsg::Mint {
        token_id: 3, // 使用不同的 token_id
        owner: "user1".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "series_3".to_string(),
            collection_group_id: None,
            serial_in_series: 1,
        },
    };
    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg2,
        &[],
    );
    if res.is_err() {
        println!("Final mint error: {:?}", res.as_ref().err());
    }
    assert!(res.is_ok());
}

#[test]
fn test_batch_limits() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    let nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "blind_box_contract".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 测试合成输入数量限制
    let synthesis_msg = ExecuteMsg::Synthesize {
        inputs: (1..=51).collect(), // 超过50个输入
        target: NftKind::Clover,
    };

    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &synthesis_msg,
        &[],
    );
    assert!(res.is_err());

    // 测试批量铸造限制
    let mints: Vec<luckee_nft::msg::BatchMintItem> = (1..=101).map(|i| luckee_nft::msg::BatchMintItem {
        token_id: i,
        owner: "user1".to_string(),
        extension: NftMeta {
            kind: NftKind::Clover,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: format!("series_{}", i),
            collection_group_id: None,
            serial_in_series: 1,
        },
    }).collect();

    let batch_mint_msg = ExecuteMsg::BatchMint { mints };
    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr,
        &batch_mint_msg,
        &[],
    );
    assert!(res.is_err());
}

#[test]
fn test_admin_cleanup_functions() {
    let mut app = mock_app();
    let contract_id = app.store_code(contract());
    
    let _nft_contract_addr = app.instantiate_contract(
        contract_id,
        Addr::unchecked("creator"),
        &InstantiateMsg {
            name: "Luckee NFT".to_string(),
            symbol: "LUCKEE".to_string(),
            minter: "blind_box_contract".to_string(),
            base_uri: Some("https://luckee.io/metadata/".to_string()),
        },
        &[],
        "Luckee NFT",
        None,
    ).unwrap();

    // 清理功能已被移除，所有操作都改为本地 CW721 模式
}
