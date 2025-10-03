//! 综合测试模块
//! 
//! 此模块包含针对合约核心功能的综合测试，重点测试：
//! - mint / duplicate mint 被拒 / total supply 增加
//! - transfer（包括 approvals/operator）和 approvals 被清理
//! - burn（删除 owner 索引、total supply 减少）
//! - batch_mint limits
//! - query: OwnerOf, Approvals, ApprovalsAll, Tokens (分页), AllTokens
//! - 事件与 attributes 统一性

use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, ContractWrapper, Executor};
use luckee_nft::state::Expiration;

use luckee_nft::contract::{execute, instantiate, query, migrate};
use luckee_nft::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use luckee_nft::types::{NftKind, NftMeta, Scale};

fn mock_app() -> App {
    App::default()
}

fn contract() -> Box<dyn cw_multi_test::Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query)
        .with_migrate(migrate);
    Box::new(contract)
}

#[test]
fn test_comprehensive_mint_and_supply_management() {
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

    // 测试1: 正常铸造
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
    assert!(res.is_ok());

    // 验证总供应量增加
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 1);

    // 测试2: 重复铸造应该失败
    let duplicate_mint_msg = ExecuteMsg::Mint {
        token_id: 1, // 相同的 token_id
        owner: "user2".to_string(),
        extension: NftMeta {
            kind: NftKind::Firefly,
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
        &duplicate_mint_msg,
        &[],
    );
    assert!(res.is_err());

    // 验证总供应量没有改变
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 1);

    // 测试3: 铸造第二个NFT
    let mint_msg2 = ExecuteMsg::Mint {
        token_id: 2,
        owner: "user1".to_string(),
        extension: NftMeta {
            kind: NftKind::Firefly,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "series_1".to_string(),
            collection_group_id: None,
            serial_in_series: 2,
        },
    };

    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg2,
        &[],
    );
    assert!(res.is_ok());

    // 验证总供应量再次增加
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 2);
}

#[test]
fn test_comprehensive_transfer_and_approvals_cleanup() {
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

    // 铸造NFT
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

    app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    ).unwrap();

    // 测试1: 批准操作
    let approve_msg = ExecuteMsg::Approve {
        spender: "user2".to_string(),
        token_id: 1,
        expires: Some(Expiration { at_height: None, at_time: None }),
    };

    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &approve_msg,
        &[],
    );
    assert!(res.is_ok());

    // 验证批准存在
    let query_msg = QueryMsg::Approvals { 
        token_id: 1,
        include_expired: Some(false),
    };
    let res: cw721::ApprovalsResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.approvals.len(), 1);
    assert_eq!(res.approvals[0].spender, "user2");

    // 测试2: 转移NFT
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: "user3".to_string(),
        token_id: 1,
    };

    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &transfer_msg,
        &[],
    );
    assert!(res.is_ok());

    // 验证所有权转移
    let query_msg = QueryMsg::OwnerOf { 
        token_id: 1,
        include_expired: Some(false),
    };
    let res: cw721::OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.owner, "user3");

    // 验证批准被清理
    let query_msg = QueryMsg::Approvals { 
        token_id: 1,
        include_expired: Some(false),
    };
    let res: cw721::ApprovalsResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.approvals.len(), 0);

    // 测试3: 操作员转移
    // 先铸造另一个NFT给user3
    let mint_msg2 = ExecuteMsg::Mint {
        token_id: 2,
        owner: "user3".to_string(),
        extension: NftMeta {
            kind: NftKind::Firefly,
            scale_origin: Scale::Tiny,
            physical_sku: None,
            crafted_from: None,
            series_id: "series_1".to_string(),
            collection_group_id: None,
            serial_in_series: 2,
        },
    };

    app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg2,
        &[],
    ).unwrap();

    // 批准user4为操作员
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: "user4".to_string(),
        expires: Some(Expiration { at_height: None, at_time: None }),
    };

    let res = app.execute_contract(
        Addr::unchecked("user3"),
        nft_contract_addr.clone(),
        &approve_all_msg,
        &[],
    );
    assert!(res.is_ok());

    // 操作员转移NFT
    let transfer_msg2 = ExecuteMsg::TransferNft {
        recipient: "user5".to_string(),
        token_id: 2,
    };

    let res = app.execute_contract(
        Addr::unchecked("user4"), // 操作员执行转移
        nft_contract_addr.clone(),
        &transfer_msg2,
        &[],
    );
    assert!(res.is_ok());

    // 验证所有权转移
    let query_msg = QueryMsg::OwnerOf { 
        token_id: 2,
        include_expired: Some(false),
    };
    let res: cw721::OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.owner, "user5");
}

#[test]
fn test_comprehensive_burn_and_cleanup() {
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

    // 铸造NFT
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

    app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    ).unwrap();

    // 批准操作
    let approve_msg = ExecuteMsg::Approve {
        spender: "user2".to_string(),
        token_id: 1,
        expires: Some(Expiration { at_height: None, at_time: None }),
    };

    app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &approve_msg,
        &[],
    ).unwrap();

    // 验证初始状态
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 1);

    // 销毁NFT
    let burn_msg = ExecuteMsg::Burn { token_id: 1 };

    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &burn_msg,
        &[],
    );
    assert!(res.is_ok());

    // 验证总供应量减少
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 0);

    // 验证NFT不再存在
    let query_msg = QueryMsg::OwnerOf { 
        token_id: 1,
        include_expired: Some(false),
    };
    let res = app.wrap().query_wasm_smart::<cw721::OwnerOfResponse>(
        nft_contract_addr.clone(), 
        &query_msg
    );
    assert!(res.is_err());

    // 验证批准被清理
    let query_msg = QueryMsg::Approvals { 
        token_id: 1,
        include_expired: Some(false),
    };
    let res = app.wrap().query_wasm_smart::<cw721::ApprovalsResponse>(
        nft_contract_addr.clone(), 
        &query_msg
    );
    assert!(res.is_err());
}

#[test]
fn test_comprehensive_batch_mint_limits() {
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

    // 测试正常批量铸造
    let batch_items: Vec<_> = (1..=10).map(|i| luckee_nft::msg::BatchMintItem {
        token_id: i,
        owner: format!("user{}", i),
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

    let batch_msg = ExecuteMsg::BatchMint { mints: batch_items };

    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &batch_msg,
        &[],
    );
    assert!(res.is_ok());

    // 验证总供应量
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 10);

    // 测试超过限制的批量铸造
    let large_batch: Vec<_> = (1..=101).map(|i| luckee_nft::msg::BatchMintItem {
        token_id: i + 100,
        owner: format!("user{}", i),
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

    let large_batch_msg = ExecuteMsg::BatchMint { mints: large_batch };

    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &large_batch_msg,
        &[],
    );
    assert!(res.is_err());

    // 验证总供应量没有改变
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 10);
}

#[test]
fn test_comprehensive_query_pagination() {
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

    // 铸造多个NFT给同一个用户
    for i in 1..=5 {
        let mint_msg = ExecuteMsg::Mint {
            token_id: i,
            owner: "user1".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "series_1".to_string(),
                collection_group_id: None,
                serial_in_series: i,
            },
        };

        app.execute_contract(
            Addr::unchecked("blind_box_contract"),
            nft_contract_addr.clone(),
            &mint_msg,
            &[],
        ).unwrap();
    }

    // 测试分页查询用户拥有的NFT
    let query_msg = QueryMsg::Tokens {
        owner: "user1".to_string(),
        start_after: None,
        limit: Some(3),
    };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 3);
    assert_eq!(res.tokens, vec!["1", "2", "3"]);

    // 查询下一页
    let query_msg = QueryMsg::Tokens {
        owner: "user1".to_string(),
        start_after: Some(3),
        limit: Some(3),
    };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 2);
    assert_eq!(res.tokens, vec!["4", "5"]);

    // 测试分页查询所有NFT
    let query_msg = QueryMsg::AllTokens {
        start_after: None,
        limit: Some(3),
    };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 3);
    assert_eq!(res.tokens, vec!["1", "2", "3"]);

    // 查询下一页
    let query_msg = QueryMsg::AllTokens {
        start_after: Some(3),
        limit: Some(3),
    };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 2);
    assert_eq!(res.tokens, vec!["4", "5"]);
}

#[test]
fn test_comprehensive_events_and_attributes() {
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

    // 测试铸造事件
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
    ).unwrap();

    // 验证事件属性
    assert!(res.events.len() >= 1);
    let mint_event = &res.events[0];
    assert_eq!(mint_event.ty, "wasm");
    
    // 验证属性键的一致性
    let action_attr = mint_event.attributes.iter()
        .find(|attr| attr.key == "action")
        .unwrap();
    assert_eq!(action_attr.value, "mint");

    let token_id_attr = mint_event.attributes.iter()
        .find(|attr| attr.key == "token_id")
        .unwrap();
    assert_eq!(token_id_attr.value, "1");

    let owner_attr = mint_event.attributes.iter()
        .find(|attr| attr.key == "owner")
        .unwrap();
    assert_eq!(owner_attr.value, "user1");

    // 测试转移事件
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: "user2".to_string(),
        token_id: 1,
    };

    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &transfer_msg,
        &[],
    ).unwrap();

    // 验证转移事件属性
    assert!(res.events.len() >= 1);
    let transfer_event = &res.events[0];
    assert_eq!(transfer_event.ty, "wasm");
    
    let action_attr = transfer_event.attributes.iter()
        .find(|attr| attr.key == "action")
        .unwrap();
    assert_eq!(action_attr.value, "transfer");

    let from_attr = transfer_event.attributes.iter()
        .find(|attr| attr.key == "from")
        .unwrap();
    assert_eq!(from_attr.value, "user1");

    let to_attr = transfer_event.attributes.iter()
        .find(|attr| attr.key == "to")
        .unwrap();
    assert_eq!(to_attr.value, "user2");

    // 测试销毁事件
    let burn_msg = ExecuteMsg::Burn { token_id: 1 };

    let res = app.execute_contract(
        Addr::unchecked("user2"),
        nft_contract_addr.clone(),
        &burn_msg,
        &[],
    ).unwrap();

    // 验证销毁事件属性
    assert!(res.events.len() >= 1);
    let burn_event = &res.events[0];
    assert_eq!(burn_event.ty, "wasm");
    
    let action_attr = burn_event.attributes.iter()
        .find(|attr| attr.key == "action")
        .unwrap();
    assert_eq!(action_attr.value, "burn");

    let owner_attr = burn_event.attributes.iter()
        .find(|attr| attr.key == "owner")
        .unwrap();
    assert_eq!(owner_attr.value, "user2");
}

#[test]
fn test_comprehensive_checked_arithmetic() {
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

    // 铸造NFT
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

    app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    ).unwrap();

    // 验证初始状态
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 1);

    // 销毁NFT
    let burn_msg = ExecuteMsg::Burn { token_id: 1 };

    app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &burn_msg,
        &[],
    ).unwrap();

    // 验证总供应量减少
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 0);

    // 再次销毁应该失败（NFT不存在）
    let burn_msg2 = ExecuteMsg::Burn { token_id: 1 };

    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &burn_msg2,
        &[],
    );
    assert!(res.is_err());

    // 验证总供应量没有改变
    let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
    let res: cw721::TokensResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.tokens.len(), 0);
}
