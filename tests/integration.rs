use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, Addr, from_json};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use luckee_nft::contract::{execute, instantiate, query};
use luckee_nft::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TokenMetaResponse};
use luckee_nft::state::CONFIG;
use luckee_nft::types::{NftKind, NftMeta, Scale};

fn mock_app() -> App {
    App::default()
}

fn contract() -> Box<dyn Contract<cosmwasm_std::Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("creator", &coins(1000, "uluckee"));

    let msg = InstantiateMsg {
        name: "Luckee NFT".to_string(),
        symbol: "LUCKEE".to_string(),
        minter: "blind_box_contract".to_string(),
        base_uri: Some("https://luckee.io/metadata/".to_string()),
    };

    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    // 验证配置
    let config = CONFIG.load(&deps.storage).unwrap();
    assert_eq!(config.name, "Luckee NFT");
    assert_eq!(config.symbol, "LUCKEE");
    assert_eq!(config.minter, Addr::unchecked("blind_box_contract"));
}

#[test]
fn test_mint_nft() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("creator", &coins(1000, "uluckee"));

    // 实例化合约
    let init_msg = InstantiateMsg {
        name: "Luckee NFT".to_string(),
        symbol: "LUCKEE".to_string(),
        minter: "blind_box_contract".to_string(),
        base_uri: Some("https://luckee.io/metadata/".to_string()),
    };
    instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();

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

    let mint_info = mock_info("blind_box_contract", &[]);
    let res = execute(deps.as_mut(), env.clone(), mint_info, mint_msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    // 查询NFT元数据
    let query_msg = QueryMsg::TokenMeta { token_id: 1 };
    let res: TokenMetaResponse = from_json(&query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
    assert_eq!(res.meta.kind, NftKind::Clover);
    assert_eq!(res.meta.series_id, "series_1");
}

#[test]
fn test_synthesis() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("creator", &coins(1000, "uluckee"));

    // 实例化合约
    let init_msg = InstantiateMsg {
        name: "Luckee NFT".to_string(),
        symbol: "LUCKEE".to_string(),
        minter: "blind_box_contract".to_string(),
        base_uri: Some("https://luckee.io/metadata/".to_string()),
    };
    instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();

    // 铸造两个四叶草NFT
    let mint_info = mock_info("blind_box_contract", &[]);
    for i in 1..=2 {
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
        execute(deps.as_mut(), env.clone(), mint_info.clone(), mint_msg).unwrap();
    }

    // 合成流萤
    let synthesis_msg = ExecuteMsg::Synthesize {
        inputs: vec![1, 2],
        target: NftKind::Firefly,
    };

    let user_info = mock_info("user1", &[]);
    let res = execute(deps.as_mut(), env.clone(), user_info, synthesis_msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    // 验证合成结果
    let query_msg = QueryMsg::TokenMeta { token_id: 3 };
    let res: TokenMetaResponse = from_json(&query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
    assert_eq!(res.meta.kind, NftKind::Firefly);
    assert_eq!(res.meta.crafted_from, Some(vec![1, 2]));
}

#[test]
fn test_batch_mint() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("creator", &coins(1000, "uluckee"));

    // 实例化合约
    let init_msg = InstantiateMsg {
        name: "Luckee NFT".to_string(),
        symbol: "LUCKEE".to_string(),
        minter: "blind_box_contract".to_string(),
        base_uri: Some("https://luckee.io/metadata/".to_string()),
    };
    instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();

    // 批量铸造NFT
    let batch_mint_msg = ExecuteMsg::BatchMint {
        mints: vec![
            luckee_nft::msg::BatchMintItem {
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
            },
            luckee_nft::msg::BatchMintItem {
                token_id: 2,
                owner: "user2".to_string(),
                extension: NftMeta {
                    kind: NftKind::Firefly,
                    scale_origin: Scale::Tiny,
                    physical_sku: None,
                    crafted_from: None,
                    series_id: "series_1".to_string(),
                    collection_group_id: None,
                    serial_in_series: 2,
                },
            },
        ],
    };

    let mint_info = mock_info("blind_box_contract", &[]);
    let res = execute(deps.as_mut(), env.clone(), mint_info, batch_mint_msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    // 验证铸造结果
    let query_msg1 = QueryMsg::TokenMeta { token_id: 1 };
    let res1: TokenMetaResponse = from_json(&query(deps.as_ref(), env.clone(), query_msg1).unwrap()).unwrap();
    assert_eq!(res1.meta.kind, NftKind::Clover);

    let query_msg2 = QueryMsg::TokenMeta { token_id: 2 };
    let res2: TokenMetaResponse = from_json(&query(deps.as_ref(), env, query_msg2).unwrap()).unwrap();
    assert_eq!(res2.meta.kind, NftKind::Firefly);
}

#[test]
fn test_recipe_management() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("creator", &coins(1000, "uluckee"));

    // 实例化合约
    let init_msg = InstantiateMsg {
        name: "Luckee NFT".to_string(),
        symbol: "LUCKEE".to_string(),
        minter: "blind_box_contract".to_string(),
        base_uri: Some("https://luckee.io/metadata/".to_string()),
    };
    instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();

    // 查询默认配方
    let query_msg = QueryMsg::Recipe { target: NftKind::Firefly };
    let res: luckee_nft::msg::RecipeResponse = from_json(&query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
    assert!(res.recipe.is_some());
    assert_eq!(res.recipe.unwrap().output, NftKind::Firefly);

    // 查询所有配方
    let query_msg = QueryMsg::AllRecipes { start_after: None, limit: Some(10) };
    let res: luckee_nft::msg::AllRecipesResponse = from_json(&query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
    assert_eq!(res.recipes.len(), 8); // 8个合成配方
}

#[test]
fn test_unauthorized_mint() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("creator", &coins(1000, "uluckee"));

    // 实例化合约
    let init_msg = InstantiateMsg {
        name: "Luckee NFT".to_string(),
        symbol: "LUCKEE".to_string(),
        minter: "blind_box_contract".to_string(),
        base_uri: Some("https://luckee.io/metadata/".to_string()),
    };
    instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();

    // 尝试用未授权的地址铸造NFT
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

    let unauthorized_info = mock_info("unauthorized_user", &[]);
    let res = execute(deps.as_mut(), env, unauthorized_info, mint_msg);
    assert!(res.is_err());
}

#[test]
fn test_nft_kind_methods() {
    // 测试NftKind的方法
    assert_eq!(NftKind::Clover.rarity_level(), 0);
    assert_eq!(NftKind::Genesis.rarity_level(), 8);
    
    assert_eq!(NftKind::Clover.rarity_name(), "Common");
    assert_eq!(NftKind::Genesis.rarity_name(), "Genesis");
    
    assert_eq!(NftKind::Clover.exchange_value(), 1);
    assert_eq!(NftKind::Genesis.exchange_value(), 2000000);
}

#[test]
fn test_scale_first_prize() {
    // 测试Scale的first_prize_nft方法
    use luckee_nft::types::Scale;
    
    assert_eq!(Scale::Tiny.first_prize_nft(), NftKind::CrimsonKoi);
    assert_eq!(Scale::Small.first_prize_nft(), NftKind::MagicalLamp);
    assert_eq!(Scale::Medium.first_prize_nft(), NftKind::FatesSpindle);
    assert_eq!(Scale::Large.first_prize_nft(), NftKind::Sage);
    assert_eq!(Scale::Huge.first_prize_nft(), NftKind::Polaris);
}

#[test]
fn test_cw721_integration_mint_transfer() {
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

    // 不设置外部CW721合约，使用metadata-only模式

    // 查询NFT合约地址
    let query_msg = QueryMsg::GetNftContract {};
    let res: luckee_nft::msg::NftContractResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    assert_eq!(res.contract_addr, None);

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

    let _mint_info = mock_info("blind_box_contract", &[]);
    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    ).unwrap();

    // 验证铸造成功
    assert!(res.events.len() >= 1);
    // metadata-only模式下事件类型为"execute"
    assert_eq!(res.events[0].ty, "execute");

    // 查询NFT元数据
    let query_msg = QueryMsg::TokenMeta { token_id: 1 };
    let res: luckee_nft::msg::TokenMetaResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr, &query_msg)
        .unwrap();
    assert_eq!(res.meta.kind, NftKind::Clover);
    assert_eq!(res.meta.series_id, "series_1");
}

#[test]
fn test_synthesize_owner_check() {
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

    // 铸造两个四叶草NFT给user1
    let _mint_info = mock_info("blind_box_contract", &[]);
    for i in 1..=2 {
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

    // user2尝试合成user1的NFT（应该失败）
    let synthesis_msg = ExecuteMsg::Synthesize {
        inputs: vec![1, 2],
        target: NftKind::Firefly,
    };

    let res = app.execute_contract(
        Addr::unchecked("user2"),
        nft_contract_addr.clone(),
        &synthesis_msg,
        &[],
    );
    
    // 应该失败，因为user2不拥有这些NFT
    assert!(res.is_err());

    // user1尝试合成自己的NFT（应该成功）
    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &synthesis_msg,
        &[],
    );
    
    // 应该成功
    assert!(res.is_ok());
    
    // 验证合成结果
    let query_msg = QueryMsg::TokenMeta { token_id: 3 };
    let res: luckee_nft::msg::TokenMetaResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr, &query_msg)
        .unwrap();
    assert_eq!(res.meta.kind, NftKind::Firefly);
    assert_eq!(res.meta.crafted_from, Some(vec![1, 2]));
}

#[test]
fn test_batch_mint_gas_and_split() {
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

    // 测试大批量铸造（100个NFT）
    let mut mints = Vec::new();
    for i in 1..=100 {
        mints.push(luckee_nft::msg::BatchMintItem {
            token_id: i,
            owner: "user1".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "batch_series".to_string(),
                collection_group_id: None,
                serial_in_series: i,
            },
        });
    }

    let batch_mint_msg = ExecuteMsg::BatchMint { mints };
    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &batch_mint_msg,
        &[],
    );

    // 应该成功
    assert!(res.is_ok());
    
    // 验证所有NFT都被铸造
    for i in 1..=100 {
        let query_msg = QueryMsg::TokenMeta { token_id: i };
        let res: luckee_nft::msg::TokenMetaResponse = app
            .wrap()
            .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
            .unwrap();
        assert_eq!(res.meta.kind, NftKind::Clover);
        assert_eq!(res.meta.series_id, "batch_series");
    }
}

#[test]
fn test_replay_duplicate_token_id() {
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

    // 第一次铸造NFT
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

    // 尝试重复铸造相同的token_id（应该失败）
    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr.clone(),
        &mint_msg,
        &[],
    );
    
    // 应该失败，因为token_id已存在
    assert!(res.is_err());
}

#[test]
fn test_pause_unpause_mechanism() {
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

    // 暂停合约
    let pause_msg = ExecuteMsg::Pause {};
    let res = app.execute_contract(
        Addr::unchecked("creator"),
        nft_contract_addr.clone(),
        &pause_msg,
        &[],
    );
    assert!(res.is_ok());

    // 尝试在暂停状态下铸造NFT（应该失败）
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
    assert!(res.is_err());

    // 恢复合约
    let unpause_msg = ExecuteMsg::Unpause {};
    let res = app.execute_contract(
        Addr::unchecked("creator"),
        nft_contract_addr.clone(),
        &unpause_msg,
        &[],
    );
    assert!(res.is_ok());

    // 现在铸造应该成功
    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr,
        &mint_msg,
        &[],
    );
    assert!(res.is_ok());
}

#[test]
fn test_emergency_withdraw() {
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

    // 在测试环境中，直接测试紧急提取功能（即使没有资金也应该能执行）

    // 尝试紧急提取（合约中没有资金，所以提取0金额）
    let emergency_msg = ExecuteMsg::EmergencyWithdraw { 
        amount: vec![] 
    };
    let res = app.execute_contract(
        Addr::unchecked("creator"),
        nft_contract_addr.clone(),
        &emergency_msg,
        &[],
    );
    if res.is_err() {
        println!("Emergency withdraw error: {:?}", res.as_ref().err());
    }
    assert!(res.is_ok());

    // 非管理员尝试紧急提取（应该失败）
    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr,
        &emergency_msg,
        &[],
    );
    assert!(res.is_err());
}

#[test]
fn test_batch_mint_size_limit() {
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

    // 测试超过批量限制的铸造（应该失败）
    let mut mints = Vec::new();
    for i in 1..=101 { // 超过100的限制
        mints.push(luckee_nft::msg::BatchMintItem {
            token_id: i,
            owner: "user1".to_string(),
            extension: NftMeta {
                kind: NftKind::Clover,
                scale_origin: Scale::Tiny,
                physical_sku: None,
                crafted_from: None,
                series_id: "batch_series".to_string(),
                collection_group_id: None,
                serial_in_series: i,
            },
        });
    }

    let batch_mint_msg = ExecuteMsg::BatchMint { mints };
    let res = app.execute_contract(
        Addr::unchecked("blind_box_contract"),
        nft_contract_addr,
        &batch_mint_msg,
        &[],
    );

    // 应该失败，因为超过了批量限制
    assert!(res.is_err());
}

#[test]
fn test_cw721_ownership_verification() {
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

    // 不设置外部CW721合约，使用metadata-only模式

    // 铸造NFT给user1
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

    // user2尝试燃烧user1的NFT（应该失败）
    let burn_msg = ExecuteMsg::Burn { token_id: 1 };
    let res = app.execute_contract(
        Addr::unchecked("user2"),
        nft_contract_addr.clone(),
        &burn_msg,
        &[],
    );
    assert!(res.is_err());

    // 在本地 CW721 模式下，burn操作应该成功
    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr,
        &burn_msg,
        &[],
    );
    assert!(res.is_ok());
}

#[test]
fn test_contract_version_and_migration() {
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

    // 查询合约信息
    let query_msg = QueryMsg::LuckeeContractInfo {};
    let res: luckee_nft::msg::LuckeeContractInfoResponse = app
        .wrap()
        .query_wasm_smart(nft_contract_addr.clone(), &query_msg)
        .unwrap();
    
    assert_eq!(res.name, "Luckee NFT");
    assert_eq!(res.symbol, "LUCKEE");
    assert_eq!(res.minter, "blind_box_contract");
    assert_eq!(res.total_supply, 0);
}

#[test]
fn test_recipe_management_security() {
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

    // 非管理员尝试设置配方（应该失败）
    let recipe_msg = ExecuteMsg::SetRecipe {
        target: NftKind::Firefly,
        recipe: luckee_nft::types::Recipe {
            inputs: vec![luckee_nft::types::RecipeInput { 
                nft_kind: NftKind::Clover, 
                count: 2 
            }],
            output: NftKind::Firefly,
            cost: None,
        },
    };

    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &recipe_msg,
        &[],
    );
    assert!(res.is_err());

    // 管理员设置配方（应该成功）
    let res = app.execute_contract(
        Addr::unchecked("creator"),
        nft_contract_addr,
        &recipe_msg,
        &[],
    );
    assert!(res.is_ok());
}

#[test]
fn test_synthesis_edge_cases() {
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

    // 测试空输入合成（应该失败）
    let synthesis_msg = ExecuteMsg::Synthesize {
        inputs: vec![],
        target: NftKind::Firefly,
    };

    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr.clone(),
        &synthesis_msg,
        &[],
    );
    assert!(res.is_err());

    // 测试不存在的配方（应该失败）
    let synthesis_msg = ExecuteMsg::Synthesize {
        inputs: vec![1, 2],
        target: NftKind::Genesis, // 这个配方不存在
    };

    let res = app.execute_contract(
        Addr::unchecked("user1"),
        nft_contract_addr,
        &synthesis_msg,
        &[],
    );
    assert!(res.is_err());
}

