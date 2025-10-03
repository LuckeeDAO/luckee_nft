//! 辅助函数模块
//! 
//! 此模块包含各种通用的辅助函数，用于：
//! - 合约状态检查
//! - 权限验证
//! - 数据验证
//! - 索引维护

use cosmwasm_std::{Addr, Deps, Storage};
use crate::error::ContractError;
use crate::state::TOKEN_APPROVALS;
use crate::state::{Config, CONTRACT_PAUSED, ALLOWED_MINTERS, TOKEN_META, TOKEN_OWNERSHIP, TOKENS_BY_OWNER};
use crate::types::Recipe;

// ========== 状态检查函数 ==========

/// 检查合约是否暂停
/// 
/// 验证合约是否处于暂停状态，如果是则返回错误
/// 
/// # 参数
/// - `storage`: 存储接口
/// 
/// # 返回值
/// - `Result<(), ContractError>`: 检查结果
pub fn check_contract_paused(storage: &dyn Storage) -> Result<(), ContractError> {
    let is_paused = CONTRACT_PAUSED.load(storage).unwrap_or(false);
    if is_paused {
        return Err(ContractError::ContractPaused {});
    }
    Ok(())
}

// ========== 权限验证函数 ==========

/// 验证铸造权限
/// 
/// 检查指定地址是否有铸造 NFT 的权限
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `sender`: 发送者地址
/// - `config`: 合约配置
/// 
/// # 返回值
/// - `Result<bool, ContractError>`: 是否有铸造权限
pub fn is_authorized_minter(deps: Deps, sender: &Addr, config: &Config) -> Result<bool, ContractError> {
    // 检查是否是合约配置的主要铸造者
    if sender == &config.minter {
        return Ok(true);
    }

    // 检查是否在允许的铸造者列表中
    if let Ok(allowed) = ALLOWED_MINTERS.may_load(deps.storage, sender.clone()) {
        return Ok(allowed.unwrap_or(false));
    }

    Ok(false)
}

/// 验证 NFT 所有权
/// 
/// 检查指定地址是否拥有指定的 NFT
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `token_id`: NFT ID
/// - `owner`: 要验证的所有者地址
/// 
/// # 返回值
/// - `Result<bool, ContractError>`: 是否拥有该 NFT
pub fn verify_nft_ownership(
    deps: Deps,
    token_id: u64,
    owner: &Addr,
) -> Result<bool, ContractError> {
    // 使用原生 CW721 存储验证所有权
    let token_owner = TOKEN_OWNERSHIP.load(deps.storage, token_id)?;
    Ok(token_owner == *owner)
}

// ========== 数据验证函数 ==========

/// 验证合成输入
/// 
/// 验证合成操作的输入 NFT 是否有效且符合配方要求
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `sender`: 发送者地址
/// - `inputs`: 输入 NFT ID 列表
/// - `recipe`: 合成配方
/// 
/// # 返回值
/// - `Result<(), ContractError>`: 验证结果
pub fn validate_synthesis_inputs(
    deps: Deps,
    sender: &Addr,
    inputs: &[u64],
    recipe: &Recipe,
) -> Result<(), ContractError> {
    // 检查输入数量
    if inputs.is_empty() {
        return Err(ContractError::InsufficientInputTokens {});
    }

    // 预先加载所有输入NFT的元数据，避免重复读取
    let mut input_metas = alloc::collections::BTreeMap::new();
    for token_id in inputs {
        let meta = TOKEN_META.may_load(deps.storage, *token_id)?;
        if meta.is_none() {
            return Err(ContractError::TokenNotFound {});
        }
        
        // 验证 CW721 所有权
        if !verify_nft_ownership(deps, *token_id, sender)? {
            return Err(ContractError::NotOwned {});
        }
        
        // 缓存元数据供后续使用
        if let Some(meta) = meta {
            input_metas.insert(*token_id, meta);
        }
    }

    // 验证配方要求（使用缓存的元数据）
    for recipe_input in &recipe.inputs {
        let count = inputs.iter()
            .filter(|&&token_id| {
                input_metas.get(&token_id)
                    .map(|meta| meta.kind == recipe_input.nft_kind)
                    .unwrap_or(false)
            })
            .count();

        if count < recipe_input.count as usize {
            return Err(ContractError::InsufficientInputTokens {});
        }
    }

    Ok(())
}

// ========== 数据验证函数 ==========

/// 验证系列ID格式
/// 
/// 检查系列ID是否符合格式要求
/// 
/// # 参数
/// - `series_id`: 要验证的系列ID
/// 
/// # 返回值
/// - `Result<(), ContractError>`: 验证结果
pub fn validate_series_id(series_id: &str) -> Result<(), ContractError> {
    // 检查是否为空
    if series_id.is_empty() {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Series ID cannot be empty")));
    }
    
    // 检查长度限制（避免过长的ID）
    if series_id.len() > 100 {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Series ID too long")));
    }
    
    // 检查字符集（只允许字母、数字、下划线、连字符）
    if !series_id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Series ID contains invalid characters")));
    }
    
    Ok(())
}

/// 验证集合组ID格式
/// 
/// 检查集合组ID是否符合格式要求
/// 
/// # 参数
/// - `group_id`: 要验证的集合组ID
/// 
/// # 返回值
/// - `Result<(), ContractError>`: 验证结果
pub fn validate_collection_group_id(group_id: &str) -> Result<(), ContractError> {
    // 检查是否为空
    if group_id.is_empty() {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Collection group ID cannot be empty")));
    }
    
    // 检查长度限制
    if group_id.len() > 100 {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Collection group ID too long")));
    }
    
    // 检查字符集
    if !group_id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(ContractError::Std(cosmwasm_std::StdError::generic_err("Collection group ID contains invalid characters")));
    }
    
    Ok(())
}

// ========== 类型转换函数 ==========

/// 将字符串 token_id 转换为 u64
/// 
/// 用于外部接口接收字符串格式的 token_id 时进行转换
/// 
/// # 参数
/// - `token_id_str`: 字符串格式的 token_id
/// 
/// # 返回值
/// - `Result<u64, ContractError>`: 转换结果
pub fn parse_token_id(token_id_str: &str) -> Result<u64, ContractError> {
    token_id_str.parse::<u64>()
        .map_err(|_| ContractError::Std(cosmwasm_std::StdError::parse_err("u64", token_id_str)))
}

/// 将 u64 token_id 转换为字符串
/// 
/// 用于响应中返回字符串格式的 token_id
/// 
/// # 参数
/// - `token_id`: u64 格式的 token_id
/// 
/// # 返回值
/// - `String`: 字符串格式的 token_id
pub fn format_token_id(token_id: u64) -> String {
    token_id.to_string()
}

// ========== 索引维护函数 ==========

/// 清理指定 NFT 的所有批准信息
/// 
/// 在 NFT 转移或销毁时调用，确保不会遗留无效的批准信息
/// 
/// # 参数
/// - `storage`: 存储接口
/// - `token_id`: 要清理批准的 NFT ID
/// 
/// # 返回值
/// - `Result<(), ContractError>`: 清理结果
pub fn clear_token_approvals(
    storage: &mut dyn Storage,
    token_id: u64,
) -> Result<(), ContractError> {
    // 移除该 NFT 的所有批准信息
    TOKEN_APPROVALS.remove(storage, token_id);
    Ok(())
}

/// 更新所有者索引
/// 
/// 在 NFT 所有权转移时更新所有者索引
/// 
/// # 参数
/// - `storage`: 存储接口
/// - `from`: 原所有者地址
/// - `to`: 新所有者地址
/// - `token_id`: NFT ID
/// 
/// # 返回值
/// - `Result<(), ContractError>`: 更新结果
pub fn update_owner_tokens(
    storage: &mut dyn Storage,
    from: &Addr,
    to: &Addr,
    token_id: u64,
) -> Result<(), ContractError> {
    // 从原所有者的索引中移除
    if let Some(mut tokens) = TOKENS_BY_OWNER.may_load(storage, from.clone())? {
        tokens.retain(|&id| id != token_id);
        if tokens.is_empty() {
            TOKENS_BY_OWNER.remove(storage, from.clone());
        } else {
            TOKENS_BY_OWNER.save(storage, from.clone(), &tokens)?;
        }
    }

    // 添加到新所有者的索引中
    let mut tokens = TOKENS_BY_OWNER.may_load(storage, to.clone())?.unwrap_or_default();
    tokens.push(token_id);
    tokens.sort(); // 保持有序以便分页查询
    TOKENS_BY_OWNER.save(storage, to.clone(), &tokens)?;

    Ok(())
}

/// 添加 NFT 到所有者索引
/// 
/// 将新铸造的 NFT 添加到指定所有者的索引中
/// 
/// # 参数
/// - `storage`: 存储接口
/// - `owner`: 所有者地址
/// - `token_id`: NFT ID
/// 
/// # 返回值
/// - `Result<(), ContractError>`: 添加结果
pub fn add_token_to_owner(
    storage: &mut dyn Storage,
    owner: &Addr,
    token_id: u64,
) -> Result<(), ContractError> {
    // 获取现有索引或创建新索引
    let mut tokens = TOKENS_BY_OWNER.may_load(storage, owner.clone())?.unwrap_or_default();
    tokens.push(token_id);
    tokens.sort(); // 保持有序以便分页查询
    TOKENS_BY_OWNER.save(storage, owner.clone(), &tokens)?;
    Ok(())
}

/// 从所有者索引中移除 NFT
/// 
/// 将销毁的 NFT 从指定所有者的索引中移除
/// 
/// # 参数
/// - `storage`: 存储接口
/// - `owner`: 所有者地址
/// - `token_id`: NFT ID
/// 
/// # 返回值
/// - `Result<(), ContractError>`: 移除结果
pub fn remove_token_from_owner(
    storage: &mut dyn Storage,
    owner: &Addr,
    token_id: u64,
) -> Result<(), ContractError> {
    // 从所有者索引中移除指定的 NFT
    if let Some(mut tokens) = TOKENS_BY_OWNER.may_load(storage, owner.clone())? {
        tokens.retain(|&id| id != token_id);
        if tokens.is_empty() {
            // 如果索引为空，则删除整个条目
            TOKENS_BY_OWNER.remove(storage, owner.clone());
        } else {
            // 否则保存更新后的索引
            TOKENS_BY_OWNER.save(storage, owner.clone(), &tokens)?;
        }
    }
    Ok(())
}
