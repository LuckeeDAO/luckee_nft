//! Luckee 扩展功能实现模块
//! 
//! 此模块包含所有 Luckee 特有的 NFT 功能，包括：
//! - NFT 铸造和销毁
//! - 合成系统（将多个 NFT 合成为一个新的 NFT）
//! - 配方管理
//! - 批量操作
//! - 扩展查询功能

#[cfg(feature = "cosmwasm")]
use cosmwasm_std::{
    to_json_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Binary, Order,
};
#[cfg(feature = "cosmwasm")]
use cw_storage_plus::Bound;

use crate::error::ContractError;
#[cfg(feature = "cosmwasm")]
use crate::state::{
    TOKEN_META, TOKEN_OWNERSHIP, SERIES_NEXT_SERIAL, TOTAL_SUPPLY, RECIPES, 
    SYNTHESIS_HISTORY, SynthesisRecord, ALL_TOKENS, NEXT_TOKEN_ID
};
use crate::types::{NftKind, NftMeta, Recipe, Scale};
use crate::msg::{BatchMintItem, TokensByKindResponse, TokensBySeriesResponse, 
                TokensByGroupResponse, LuckeeContractInfoResponse, AllRecipesResponse, 
                SynthesisPreviewResponse, NftContractResponse};
#[cfg(feature = "cosmwasm")]
use crate::helpers::{check_contract_paused, is_authorized_minter, validate_synthesis_inputs, 
                    add_token_to_owner, validate_series_id, validate_collection_group_id};
#[cfg(feature = "cosmwasm")]
use crate::events::{emit_mint_event, emit_burn_event, emit_synthesize_event, emit_batch_mint_event};

// ========== 常量定义 ==========

/// 合成操作的最大输入数量限制
const MAX_SYNTHESIS_INPUTS: usize = 50;

/// 批量铸造的最大数量限制
const MAX_BATCH_MINT: usize = 100;



// ========== Luckee 扩展执行接口 ==========

/// 铸造新的 NFT
/// 
/// 创建新的 NFT 并分配给指定所有者
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `token_id`: 要铸造的 NFT ID
/// - `owner`: 新 NFT 的所有者地址
/// - `extension`: NFT 的扩展元数据
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 铸造结果
#[cfg(feature = "cosmwasm")]
pub fn execute_mint(
    deps: DepsMut,
    info: MessageInfo,
    token_id: u64,
    owner: String,
    extension: NftMeta,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 加载合约配置
    let config = crate::state::CONFIG.load(deps.storage)?;
    
    // 验证铸造者权限
    if !is_authorized_minter(deps.as_ref(), &info.sender, &config)? {
        return Err(ContractError::MinterNotAuthorized {});
    }

    // 验证所有者地址格式
    let owner_addr = deps.api.addr_validate(&owner)?;

    // 验证系列ID格式
    validate_series_id(&extension.series_id)?;
    
    // 验证集合组ID格式（如果提供）
    if let Some(ref group_id) = extension.collection_group_id {
        validate_collection_group_id(group_id)?;
    }

    // 检查 NFT 是否已存在
    if TOKEN_META.has(deps.storage, token_id) {
        return Err(ContractError::TokenAlreadyExists {});
    }

    // 更新NEXT_TOKEN_ID计数器，确保后续生成的ID不会冲突
    let current_next_id = NEXT_TOKEN_ID.load(deps.storage)?;
    if token_id >= current_next_id {
        NEXT_TOKEN_ID.save(deps.storage, &(token_id.checked_add(1).ok_or(ContractError::Overflow {})?))?;
    }

    // ========== 本地 CW721 模式 ==========
    // 直接保存元数据和所有权到本地存储
    
    TOKEN_META.save(deps.storage, token_id, &extension)?;
    TOKEN_OWNERSHIP.save(deps.storage, token_id, &owner_addr)?;
    
    // 更新所有者索引和全局索引
    add_token_to_owner(deps.storage, &owner_addr, token_id)?;
    ALL_TOKENS.save(deps.storage, token_id, &())?;
    
    // 更新系列序号（使用 checked_add 防止溢出）
    let next_serial = SERIES_NEXT_SERIAL.may_load(deps.storage, extension.series_id.clone())?.unwrap_or(0);
    let new_serial = next_serial.checked_add(1)
        .ok_or(ContractError::Overflow {})?;
    SERIES_NEXT_SERIAL.save(deps.storage, extension.series_id.clone(), &new_serial)?;

    // 更新总供应量（使用 checked_add 防止溢出）
    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;
    let new_supply = total_supply.checked_add(1)
        .ok_or(ContractError::Overflow {})?;
    TOTAL_SUPPLY.save(deps.storage, &new_supply)?;
    
    let owner_str = owner.clone();
    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("owner", owner)
        .add_attribute("kind", alloc::format!("{:?}", extension.kind))
        .add_event(emit_mint_event(token_id, &owner_str, &alloc::format!("{:?}", extension.kind))))
}

/// 销毁 NFT
/// 
/// 销毁指定的 NFT，根据合约配置选择不同的销毁模式
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `token_id`: 要销毁的 NFT ID
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 销毁结果
#[cfg(feature = "cosmwasm")]
pub fn execute_burn(
    deps: DepsMut,
    info: MessageInfo,
    token_id: u64,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证 NFT 是否存在
    let meta = TOKEN_META.may_load(deps.storage, token_id)?;
    if meta.is_none() {
        return Err(ContractError::TokenNotFound {});
    }

    // 验证所有者身份
    let owner = TOKEN_OWNERSHIP.load(deps.storage, token_id)?;
    if owner != info.sender {
        return Err(ContractError::NotOwned {});
    }

    // ========== 本地 CW721 模式 ==========
    // 直接删除本地元数据和所有权
    
    // 删除 NFT 元数据
    TOKEN_META.remove(deps.storage, token_id);
    TOKEN_OWNERSHIP.remove(deps.storage, token_id);
    
    // 清理销毁前的批准信息（安全措施）
    crate::helpers::clear_token_approvals(deps.storage, token_id)?;
    
    // 从所有者索引中移除
    crate::helpers::remove_token_from_owner(deps.storage, &owner, token_id)?;
    
    // 从全局索引中移除
    ALL_TOKENS.remove(deps.storage, token_id);
    
    // 更新总供应量（使用 checked_sub 防止下溢）
    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;
    let new_supply = total_supply.checked_sub(1)
        .ok_or(ContractError::Overflow {})?;
    TOTAL_SUPPLY.save(deps.storage, &new_supply)?;
    
    Ok(Response::new()
        .add_attribute("action", "burn")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("owner", owner.to_string())
        .add_event(emit_burn_event(token_id, &owner)))
}

/// 合成 NFT
/// 
/// 将多个输入 NFT 合成为一个新的目标 NFT
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `env`: 环境信息，包含区块高度和时间
/// - `info`: 消息信息，包含发送者
/// - `inputs`: 输入 NFT ID 列表
/// - `target`: 目标 NFT 类型
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 合成结果
#[cfg(feature = "cosmwasm")]
pub fn execute_synthesize(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    inputs: Vec<u64>,
    target: NftKind,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 检查输入数量限制
    if inputs.len() > MAX_SYNTHESIS_INPUTS {
        return Err(ContractError::TooManyInputs { count: inputs.len() });
    }
    
    // 获取合成配方
    let recipe = RECIPES.load(deps.storage, target.to_key())
        .map_err(|_| ContractError::RecipeNotFound {})?;

    // 验证输入 NFT 的所有权和有效性
    validate_synthesis_inputs(deps.as_ref(), &info.sender, &inputs, &recipe)?;

    // 生成新的 token ID（使用独立计数器确保唯一性）
    let next_token_id = NEXT_TOKEN_ID.load(deps.storage)?;
    let output_token_id = next_token_id;
    NEXT_TOKEN_ID.save(deps.storage, &(next_token_id.checked_add(1).ok_or(ContractError::Overflow {})?))?;

    // 创建输出 NFT 的元数据
    let output_meta = NftMeta {
        kind: target.clone(),
        scale_origin: Scale::Tiny, // 合成获得的 NFT 使用默认规模
        physical_sku: None,
        crafted_from: Some(inputs.clone()), // 记录合成来源
        series_id: alloc::format!("synthesis_{}", env.block.time.seconds()),
        collection_group_id: None,
        serial_in_series: 1,
    };

    // ========== 本地 CW721 模式 ==========
    // 直接完成合成操作，无需外部合约交互
    
    // 删除输入 NFT 的本地元数据
    for token_id in &inputs {
        TOKEN_META.remove(deps.storage, *token_id);
        TOKEN_OWNERSHIP.remove(deps.storage, *token_id);
        
        // 清理销毁前的批准信息（安全措施）
        crate::helpers::clear_token_approvals(deps.storage, *token_id)?;
        
        // 从所有者索引中移除
        crate::helpers::remove_token_from_owner(deps.storage, &info.sender, *token_id)?;
        
        // 从全局索引中移除
        ALL_TOKENS.remove(deps.storage, *token_id);
    }
    
    // 铸造输出 NFT
    TOKEN_META.save(deps.storage, output_token_id, &output_meta)?;
    TOKEN_OWNERSHIP.save(deps.storage, output_token_id, &info.sender)?;
    
    // 更新所有者索引和全局索引
    crate::helpers::add_token_to_owner(deps.storage, &info.sender, output_token_id)?;
    ALL_TOKENS.save(deps.storage, output_token_id, &())?;
    
    // 更新系列序号（使用 checked_add 防止溢出）
    let next_serial = SERIES_NEXT_SERIAL.may_load(deps.storage, output_meta.series_id.clone())?.unwrap_or(0);
    let new_serial = next_serial.checked_add(1)
        .ok_or(ContractError::Overflow {})?;
    SERIES_NEXT_SERIAL.save(deps.storage, output_meta.series_id.clone(), &new_serial)?;
    
    // 更新总供应量（输出 +1，输入 -inputs.len()）
    // 注意：TOTAL_SUPPLY只表示当前存在的NFT数量，不用于ID生成
    let new_total_supply = total_supply.checked_add(1)
        .and_then(|supply| supply.checked_sub(inputs.len() as u64))
        .ok_or(ContractError::Overflow {})?;
    TOTAL_SUPPLY.save(deps.storage, &new_total_supply)?;

    // 记录合成历史
    let synthesis_record = SynthesisRecord {
        user: info.sender.clone(),
        inputs: inputs.clone(),
        output: output_token_id,
        timestamp: env.block.time.seconds(),
    };
    SYNTHESIS_HISTORY.save(deps.storage, (info.sender.clone(), env.block.time.seconds()), &synthesis_record)?;

    Ok(Response::new()
        .add_attribute("action", "synthesize")
        .add_attribute("output_token_id", output_token_id.to_string())
        .add_attribute("target", alloc::format!("{:?}", target))
        .add_attribute("inputs_count", inputs.len().to_string())
        .add_event(emit_synthesize_event(output_token_id, &alloc::format!("{:?}", target), inputs.len(), &info.sender)))
}

/// 设置合成配方
/// 
/// 为指定的 NFT 类型设置合成配方
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `target`: 目标 NFT 类型
/// - `recipe`: 合成配方
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 设置结果
#[cfg(feature = "cosmwasm")]
pub fn execute_set_recipe(
    deps: DepsMut,
    info: MessageInfo,
    target: NftKind,
    recipe: Recipe,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证所有者权限
    let config = crate::state::CONFIG.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // 保存合成配方
    RECIPES.save(deps.storage, target.to_key(), &recipe)?;

    Ok(Response::new()
        .add_attribute("action", "set_recipe")
        .add_attribute("target", alloc::format!("{:?}", target)))
}

/// 删除合成配方
/// 
/// 删除指定 NFT 类型的合成配方
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `target`: 目标 NFT 类型
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 删除结果
pub fn execute_remove_recipe(
    deps: DepsMut,
    info: MessageInfo,
    target: NftKind,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证所有者权限
    let config = crate::state::CONFIG.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // 删除合成配方
    RECIPES.remove(deps.storage, target.to_key());

    Ok(Response::new()
        .add_attribute("action", "remove_recipe")
        .add_attribute("target", alloc::format!("{:?}", target)))
}

/// 批量铸造
#[cfg(feature = "cosmwasm")]
pub fn execute_batch_mint(
    deps: DepsMut,
    info: MessageInfo,
    mints: Vec<BatchMintItem>,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    let config = crate::state::CONFIG.load(deps.storage)?;
    
    // 检查铸造权限
    if !is_authorized_minter(deps.as_ref(), &info.sender, &config)? {
        return Err(ContractError::MinterNotAuthorized {});
    }

    // 批量操作安全控制：限制批量大小
    if mints.len() > MAX_BATCH_MINT {
        return Err(ContractError::TooManyTokens { count: mints.len() });
    }

    let mint_count = mints.len();
    let mut total_supply = TOTAL_SUPPLY.load(deps.storage)?;
    let mut response = Response::new()
        .add_attribute("action", "batch_mint")
        .add_attribute("count", mint_count.to_string());

    // 预先检查重复的token_id
    let mut token_ids = alloc::collections::BTreeSet::new();
    for mint_item in &mints {
        if !token_ids.insert(mint_item.token_id) {
            return Err(ContractError::TokenAlreadyExists {});
        }
        if TOKEN_META.has(deps.storage, mint_item.token_id) {
            return Err(ContractError::TokenAlreadyExists {});
        }
    }

    for mint_item in mints {
        // 校验所有者地址
        let owner_addr = deps.api.addr_validate(&mint_item.owner)?;
        
        // 验证系列ID格式
        validate_series_id(&mint_item.extension.series_id)?;
        
        // 验证集合组ID格式（如果提供）
        if let Some(ref group_id) = mint_item.extension.collection_group_id {
            validate_collection_group_id(group_id)?;
        }
        
        // 更新NEXT_TOKEN_ID计数器，确保后续生成的ID不会冲突
        let current_next_id = NEXT_TOKEN_ID.load(deps.storage)?;
        if mint_item.token_id >= current_next_id {
            NEXT_TOKEN_ID.save(deps.storage, &(mint_item.token_id.checked_add(1).ok_or(ContractError::Overflow {})?))?;
        }
        
        // 保存元数据
        TOKEN_META.save(deps.storage, mint_item.token_id, &mint_item.extension)?;
        
        // 设置所有权
        TOKEN_OWNERSHIP.save(deps.storage, mint_item.token_id, &owner_addr)?;
        
        // 更新所有者索引
        add_token_to_owner(deps.storage, &owner_addr, mint_item.token_id)?;
        
        // 添加到全局索引
        ALL_TOKENS.save(deps.storage, mint_item.token_id, &())?;
        
        // 更新系列序号（使用checked_add）
        let next_serial = SERIES_NEXT_SERIAL.may_load(deps.storage, mint_item.extension.series_id.clone())?.unwrap_or(0);
        let new_serial = next_serial.checked_add(1)
            .ok_or(ContractError::Overflow {})?;
        SERIES_NEXT_SERIAL.save(deps.storage, mint_item.extension.series_id.clone(), &new_serial)?;
        
        // 发出mint事件
        response = response.add_event(emit_mint_event(
            mint_item.token_id, 
            &mint_item.owner, 
            &alloc::format!("{:?}", mint_item.extension.kind)
        ));
        
        total_supply += 1;
    }

    // 更新总供应量（使用checked_add）
    let new_total_supply = total_supply.checked_add(mint_count as u64)
        .ok_or(ContractError::Overflow {})?;
    TOTAL_SUPPLY.save(deps.storage, &new_total_supply)?;
    
    // 发出批量铸造事件
    response = response.add_event(emit_batch_mint_event(mint_count, &info.sender));
    
    Ok(response)
}

/// 设置铸造者权限
pub fn execute_set_minter(
    deps: DepsMut,
    info: MessageInfo,
    minter: String,
    allowed: bool,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    let config = crate::state::CONFIG.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let minter_addr = deps.api.addr_validate(&minter)?;
    crate::state::ALLOWED_MINTERS.save(deps.storage, minter_addr, &allowed)?;

    Ok(Response::new()
        .add_attribute("action", "set_minter")
        .add_attribute("minter", minter)
        .add_attribute("allowed", allowed.to_string()))
}

// 查询函数实现
#[cfg(feature = "cosmwasm")]
pub fn query_tokens_by_kind(
    deps: Deps,
    kind: NftKind,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    
    // 使用Bound实现标准分页逻辑
    let start_bound = start_after.map(|id| Bound::exclusive(id));

    let tokens: Vec<u64> = TOKEN_META
        .range(deps.storage, start_bound, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(token_id, meta)| {
                if meta.kind == kind {
                    Some(token_id)
                } else {
                    None
                }
            })
        })
        .take(limit)
        .collect();

    to_json_binary(&TokensByKindResponse { tokens })
}

#[cfg(feature = "cosmwasm")]
pub fn query_tokens_by_series(
    deps: Deps,
    series_id: String,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    
    // 使用Bound实现标准分页逻辑
    let start_bound = start_after.map(|id| Bound::exclusive(id));

    let tokens: Vec<u64> = TOKEN_META
        .range(deps.storage, start_bound, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(token_id, meta)| {
                if meta.series_id == series_id {
                    Some(token_id)
                } else {
                    None
                }
            })
        })
        .take(limit)
        .collect();

    to_json_binary(&TokensBySeriesResponse { tokens })
}

#[cfg(feature = "cosmwasm")]
pub fn query_tokens_by_group(
    deps: Deps,
    group_id: String,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    
    // 使用Bound实现标准分页逻辑
    let start_bound = start_after.map(|id| Bound::exclusive(id));

    let tokens: Vec<u64> = TOKEN_META
        .range(deps.storage, start_bound, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(token_id, meta)| {
                if meta.collection_group_id.as_ref() == Some(&group_id) {
                    Some(token_id)
                } else {
                    None
                }
            })
        })
        .take(limit)
        .collect();

    to_json_binary(&TokensByGroupResponse { tokens })
}

#[cfg(feature = "cosmwasm")]
pub fn query_contract_info(deps: Deps) -> StdResult<Binary> {
    let config = crate::state::CONFIG.load(deps.storage)?;
    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;

    let info = LuckeeContractInfoResponse {
        name: config.name,
        symbol: config.symbol,
        minter: config.minter.to_string(),
        base_uri: config.base_uri,
        total_supply,
    };

    to_json_binary(&info)
}

#[cfg(feature = "cosmwasm")]
pub fn query_all_recipes(
    deps: Deps,
    start_after: Option<NftKind>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;

    let recipes: Vec<(String, Recipe)> = RECIPES
        .range(deps.storage, start_after.as_ref().map(|k| Bound::exclusive(k.to_key())), None, Order::Ascending)
        .take(limit)
        .collect::<StdResult<Vec<_>>>()?;

    // 转换String键为NftKind
    let recipes: Vec<(NftKind, Recipe)> = recipes.into_iter()
        .filter_map(|(key, recipe)| {
            NftKind::from_key(&key).ok().map(|kind| (kind, recipe))
        })
        .collect();
    
    to_json_binary(&AllRecipesResponse { recipes })
}

#[cfg(feature = "cosmwasm")]
pub fn query_synthesis_preview(
    deps: Deps,
    _inputs: Vec<u64>,
    target: NftKind,
) -> StdResult<Binary> {
    let recipe = RECIPES.may_load(deps.storage, target.to_key())?;
    
    let recipe = match recipe {
        Some(recipe) => recipe,
        None => {
            return to_json_binary(&SynthesisPreviewResponse {
                can_synthesize: false,
                required_inputs: vec![],
                output_value: 0,
                cost: None,
            });
        }
    };
    let output_value = target.exchange_value();

    to_json_binary(&SynthesisPreviewResponse {
        can_synthesize: true,
        required_inputs: recipe.inputs,
        output_value,
        cost: recipe.cost,
    })
}

#[cfg(feature = "cosmwasm")]
pub fn query_nft_contract(_deps: Deps) -> StdResult<Binary> {
    // 本地 CW721 模式，不依赖外部合约
    to_json_binary(&NftContractResponse {
        contract_addr: None,
    })
}
