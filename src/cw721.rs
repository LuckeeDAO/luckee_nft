//! 标准 CW721 接口实现模块
//! 
//! 此模块包含所有标准 CW721 NFT 接口的实现，包括：
//! - 转移 NFT 所有权 (TransferNft)
//! - 批准和撤销批准 (Approve/Revoke)
//! - 操作员管理 (ApproveAll/RevokeAll)
//! - 所有权和批准查询
//! - Token 枚举查询

use cosmwasm_std::{
    to_json_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Binary, Order,
};
use cw721::{OwnerOfResponse, NftInfoResponse, ApprovalsResponse, 
           OperatorResponse, TokensResponse, ContractInfoResponse, 
           Approval, Expiration as Cw721Expiration};

use crate::error::ContractError;
use crate::state::{
    TOKEN_OWNERSHIP, TOKEN_APPROVALS, OPERATOR_APPROVALS, TOKENS_BY_OWNER, 
    ALL_TOKENS, CONTRACT_INFO, CONFIG, Expiration
};
use crate::types::NftMeta;
use crate::helpers::{check_contract_paused, update_owner_tokens};
use crate::events::{
    emit_transfer_event, emit_approval_event, emit_revoke_event,
    emit_approve_all_event, emit_revoke_all_event
};

// ========== 标准 CW721 执行接口 ==========

/// 转移 NFT 所有权
/// 
/// 将指定 NFT 的所有权从当前所有者转移给接收者
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `_env`: 环境信息（未使用）
/// - `info`: 消息信息，包含发送者
/// - `recipient`: 接收者地址
/// - `token_id`: 要转移的 NFT ID
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 转移结果
pub fn execute_transfer_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    token_id: u64,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证当前所有者
    let owner = TOKEN_OWNERSHIP.load(deps.storage, token_id)?;
    if owner != info.sender {
        return Err(ContractError::NotOwned {});
    }
    
    // 验证接收者地址格式
    let recipient_addr = deps.api.addr_validate(&recipient)?;
    
    // 更新 NFT 所有权
    TOKEN_OWNERSHIP.save(deps.storage, token_id, &recipient_addr)?;
    
    // 清理转移前的批准信息（安全措施）
    crate::helpers::clear_token_approvals(deps.storage, token_id)?;
    
    // 更新所有者索引
    update_owner_tokens(deps.storage, &owner, &recipient_addr, token_id)?;
    
    // 返回成功响应并发出转移事件
    Ok(Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("from", owner.to_string())
        .add_attribute("to", recipient)
        .add_event(emit_transfer_event(token_id, &owner, &recipient_addr)))
}

/// 批准特定地址操作特定 NFT
/// 
/// 允许指定地址（spender）代表所有者操作指定的 NFT
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `spender`: 被批准的地址
/// - `token_id`: NFT ID
/// - `expires`: 批准过期时间（可选）
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 批准结果
pub fn execute_approve(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
    token_id: u64,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证所有者身份
    let owner = TOKEN_OWNERSHIP.load(deps.storage, token_id)?;
    if owner != info.sender {
        return Err(ContractError::NotOwned {});
    }
    
    // 验证被批准者地址格式
    let spender_addr = deps.api.addr_validate(&spender)?;
    
    // 获取现有的批准列表
    let mut approvals = TOKEN_APPROVALS.may_load(deps.storage, token_id)?.unwrap_or_default();
    
    // 移除现有的批准（如果存在），避免重复
    approvals.retain(|approval| approval.spender != spender_addr);
    
    // 添加新的批准
    approvals.push(crate::state::Approval {
        spender: spender_addr.clone(),
        expires: expires.clone(),
    });
    
    // 保存更新后的批准列表
    TOKEN_APPROVALS.save(deps.storage, token_id, &approvals)?;
    
    // 返回成功响应并发出批准事件
    Ok(Response::new()
        .add_attribute("action", "approve")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("owner", owner.to_string())
        .add_attribute("spender", spender)
        .add_event(emit_approval_event(token_id, &owner, &spender_addr)))
}

/// 撤销特定地址对特定 NFT 的批准
/// 
/// 取消指定地址（spender）对指定 NFT 的操作权限
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `spender`: 要撤销批准的地址
/// - `token_id`: NFT ID
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 撤销结果
pub fn execute_revoke(
    deps: DepsMut,
    info: MessageInfo,
    spender: String,
    token_id: u64,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证所有者身份
    let owner = TOKEN_OWNERSHIP.load(deps.storage, token_id)?;
    if owner != info.sender {
        return Err(ContractError::NotOwned {});
    }
    
    // 验证要撤销批准的地址格式
    let spender_addr = deps.api.addr_validate(&spender)?;
    
    // 获取现有的批准列表
    let mut approvals = TOKEN_APPROVALS.may_load(deps.storage, token_id)?.unwrap_or_default();
    
    // 移除指定地址的批准
    approvals.retain(|approval| approval.spender != spender_addr);
    
    // 如果批准列表为空，则删除整个条目；否则保存更新后的列表
    if approvals.is_empty() {
        TOKEN_APPROVALS.remove(deps.storage, token_id);
    } else {
        TOKEN_APPROVALS.save(deps.storage, token_id, &approvals)?;
    }
    
    // 返回成功响应并发出撤销事件
    Ok(Response::new()
        .add_attribute("action", "revoke")
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("owner", owner.to_string())
        .add_attribute("spender", spender)
        .add_event(emit_revoke_event(token_id, &owner, &spender_addr)))
}

/// 批准操作员管理所有 NFT
/// 
/// 允许指定地址（operator）代表所有者操作其拥有的所有 NFT
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `operator`: 操作员地址
/// - `expires`: 批准过期时间（可选）
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 批准结果
pub fn execute_approve_all(
    deps: DepsMut,
    info: MessageInfo,
    operator: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证操作员地址格式
    let operator_addr = deps.api.addr_validate(&operator)?;
    
    // 设置操作员批准（如果没有指定过期时间，则设置为永不过期）
    OPERATOR_APPROVALS.save(deps.storage, (info.sender.clone(), operator_addr.clone()), &expires.unwrap_or(Expiration {
        at_height: None,
        at_time: None,
    }))?;
    
    // 返回成功响应并发出批准事件
    Ok(Response::new()
        .add_attribute("action", "approve_all")
        .add_attribute("owner", info.sender.to_string())
        .add_attribute("operator", operator)
        .add_event(emit_approve_all_event(&info.sender, &operator_addr)))
}

/// 撤销操作员对所有 NFT 的管理权限
/// 
/// 取消指定地址（operator）对所有者所有 NFT 的操作权限
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `operator`: 要撤销权限的操作员地址
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 撤销结果
pub fn execute_revoke_all(
    deps: DepsMut,
    info: MessageInfo,
    operator: String,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证操作员地址格式
    let operator_addr = deps.api.addr_validate(&operator)?;
    
    // 移除操作员批准
    OPERATOR_APPROVALS.remove(deps.storage, (info.sender.clone(), operator_addr.clone()));
    
    // 返回成功响应并发出撤销事件
    Ok(Response::new()
        .add_attribute("action", "revoke_all")
        .add_attribute("owner", info.sender.to_string())
        .add_attribute("operator", operator)
        .add_event(emit_revoke_all_event(&info.sender, &operator_addr)))
}

// ========== 标准 CW721 查询接口 ==========
/// 查询 NFT 的所有者信息
/// 
/// 返回指定 NFT 的所有者地址和批准信息
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `env`: 环境信息，用于检查批准是否过期
/// - `token_id`: NFT ID
/// - `include_expired`: 是否包含过期的批准
/// 
/// # 返回值
/// - `StdResult<Binary>`: 所有者信息，包含地址和批准列表
pub fn query_owner_of(deps: Deps, env: Env, token_id: u64, include_expired: Option<bool>) -> StdResult<Binary> {
    // 获取 NFT 所有者
    let owner = TOKEN_OWNERSHIP.load(deps.storage, token_id)?;
    let approvals = TOKEN_APPROVALS.may_load(deps.storage, token_id)?.unwrap_or_default();
    
    // 根据参数决定是否过滤过期的批准
    let mut valid_approvals = Vec::new();
    if include_expired.unwrap_or(false) {
        valid_approvals = approvals;
    } else {
        // 过滤过期的批准
        for approval in approvals {
            if !approval.expires.as_ref().map_or(false, |exp| exp.is_expired(&env)) {
                valid_approvals.push(approval);
            }
        }
    }

    // 构建响应，转换内部批准格式为标准 CW721 格式
    to_json_binary(&OwnerOfResponse {
        owner: owner.to_string(),
        approvals: valid_approvals.into_iter().map(|a| Approval {
            spender: a.spender.to_string(),
            expires: a.expires.map_or(Cw721Expiration::Never {}, |e| {
                if let Some(height) = e.at_height {
                    Cw721Expiration::AtHeight(height)
                } else if let Some(time) = e.at_time {
                    Cw721Expiration::AtTime(cosmwasm_std::Timestamp::from_seconds(time))
                } else {
                    Cw721Expiration::Never {}
                }
            }),
        }).collect(),
    })
}

/// 查询 NFT 的详细信息
/// 
/// 返回指定 NFT 的元数据和 URI 信息
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `token_id`: NFT ID
/// 
/// # 返回值
/// - `StdResult<Binary>`: NFT 信息，包含 URI 和扩展元数据
pub fn query_nft_info(deps: Deps, token_id: u64) -> StdResult<Binary> {
    // 验证 NFT 是否存在（通过检查所有者）
    let _owner = TOKEN_OWNERSHIP.load(deps.storage, token_id)?;
    let meta = crate::state::TOKEN_META.load(deps.storage, token_id)?;
    
    // 构建 token URI（基于基础 URI 和 token ID）
    let config = CONFIG.load(deps.storage)?;
    let token_uri = config.base_uri.map(|base| format!("{}/{}", base, token_id));
    
    // 返回 NFT 信息响应
    to_json_binary(&NftInfoResponse::<NftMeta> {
        token_uri,
        extension: meta,
    })
}

/// 查询 NFT 的批准信息
/// 
/// 返回指定 NFT 的所有批准信息
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `env`: 环境信息，用于检查批准是否过期
/// - `token_id`: NFT ID
/// - `include_expired`: 是否包含过期的批准
/// 
/// # 返回值
/// - `StdResult<Binary>`: 批准信息列表
pub fn query_approvals(deps: Deps, env: Env, token_id: u64, include_expired: Option<bool>) -> StdResult<Binary> {
    // 验证 NFT 是否存在
    let _owner = TOKEN_OWNERSHIP.load(deps.storage, token_id)?;
    let approvals = TOKEN_APPROVALS.may_load(deps.storage, token_id)?.unwrap_or_default();
    
    // 根据参数决定是否过滤过期的批准
    let mut valid_approvals = Vec::new();
    if include_expired.unwrap_or(false) {
        valid_approvals = approvals;
    } else {
        // 过滤过期的批准
        for approval in approvals {
            if !approval.expires.as_ref().map_or(false, |exp| exp.is_expired(&env)) {
                valid_approvals.push(approval);
            }
        }
    }

    // 构建响应，转换内部批准格式为标准 CW721 格式
    to_json_binary(&ApprovalsResponse {
        approvals: valid_approvals.into_iter().map(|a| Approval {
            spender: a.spender.to_string(),
            expires: a.expires.map_or(Cw721Expiration::Never {}, |e| {
                if let Some(height) = e.at_height {
                    Cw721Expiration::AtHeight(height)
                } else if let Some(time) = e.at_time {
                    Cw721Expiration::AtTime(cosmwasm_std::Timestamp::from_seconds(time))
                } else {
                    Cw721Expiration::Never {}
                }
            }),
        }).collect(),
    })
}

/// 查询操作员是否被批准管理所有 NFT
/// 
/// 检查指定操作员是否被所有者批准管理其所有 NFT
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `env`: 环境信息，用于检查批准是否过期
/// - `owner`: 所有者地址
/// - `operator`: 操作员地址
/// 
/// # 返回值
/// - `StdResult<Binary>`: 操作员批准状态
pub fn query_is_approved_for_all(deps: Deps, env: Env, owner: String, operator: String) -> StdResult<Binary> {
    // 验证地址格式
    let owner_addr = deps.api.addr_validate(&owner)?;
    let operator_addr = deps.api.addr_validate(&operator)?;
    
    // 查询操作员批准状态
    let expiration = OPERATOR_APPROVALS.may_load(deps.storage, (owner_addr, operator_addr))?;
    
    // 检查批准是否有效（未过期）
    let approved = if let Some(exp) = expiration {
        !exp.is_expired(&env)
    } else {
        false
    };

    // 返回操作员批准响应
    to_json_binary(&OperatorResponse {
        approval: if approved {
            Approval {
                spender: operator.to_string(),
                expires: Cw721Expiration::Never {},
            }
        } else {
            Approval {
                spender: operator.to_string(),
                expires: Cw721Expiration::Never {},
            }
        },
    })
}

/// 查询 NFT 的 URI 信息
/// 
/// 返回指定 NFT 的 URI 和元数据信息
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `token_id`: NFT ID
/// 
/// # 返回值
/// - `StdResult<Binary>`: NFT URI 信息
pub fn query_token_uri(deps: Deps, token_id: u64) -> StdResult<Binary> {
    // 验证 NFT 是否存在
    let _owner = TOKEN_OWNERSHIP.load(deps.storage, token_id)?;
    let meta = crate::state::TOKEN_META.load(deps.storage, token_id)?;
    
    // 构建 token URI
    let config = CONFIG.load(deps.storage)?;
    let token_uri = config.base_uri.map(|base| format!("{}/{}", base, token_id));
    
    to_json_binary(&NftInfoResponse::<NftMeta> {
        token_uri,
        extension: meta,
    })
}

/// 查询所有 NFT 列表
/// 
/// 返回所有 NFT 的 ID 列表，支持分页
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `_env`: 环境信息（未使用）
/// - `start_after`: 分页起始位置
/// - `limit`: 返回数量限制
/// 
/// # 返回值
/// - `StdResult<Binary>`: NFT ID 列表
pub fn query_all_tokens(deps: Deps, _env: Env, start_after: Option<u64>, limit: Option<u32>) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30).min(30) as usize;
    let start = start_after.unwrap_or(0);

    // 获取所有 NFT ID，支持分页
    let tokens: Vec<u64> = ALL_TOKENS
        .keys(deps.storage, None, None, Order::Ascending)
        .skip_while(|token_id| {
            if let Ok(id) = token_id {
                *id <= start
            } else {
                false
            }
        })
        .skip(if start_after.is_some() { 1 } else { 0 })
        .take(limit)
        .collect::<Result<Vec<_>, _>>()?;

    to_json_binary(&TokensResponse { 
        tokens: tokens.into_iter().map(|id| id.to_string()).collect() 
    })
}

/// 查询指定用户拥有的 NFT 列表
/// 
/// 返回指定用户拥有的所有 NFT ID 列表，支持分页
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `_env`: 环境信息（未使用）
/// - `owner`: 用户地址
/// - `start_after`: 分页起始位置
/// - `limit`: 返回数量限制
/// 
/// # 返回值
/// - `StdResult<Binary>`: 用户拥有的 NFT ID 列表
pub fn query_tokens(deps: Deps, _env: Env, owner: String, start_after: Option<u64>, limit: Option<u32>) -> StdResult<Binary> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let limit = limit.unwrap_or(30).min(30) as usize;
    
    // 获取用户拥有的 NFT 列表，支持分页
    let tokens: Vec<u64> = TOKENS_BY_OWNER
        .may_load(deps.storage, owner_addr)?
        .unwrap_or_default()
        .into_iter()
        .skip_while(|token_id| {
            if let Some(start) = start_after {
                *token_id <= start
            } else {
                false
            }
        })
        .skip(if start_after.is_some() { 1 } else { 0 })
        .take(limit)
        .collect();

    to_json_binary(&TokensResponse { 
        tokens: tokens.into_iter().map(|id| id.to_string()).collect() 
    })
}

/// 查询标准 CW721 合约信息
/// 
/// 返回合约的名称和符号信息
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// 
/// # 返回值
/// - `StdResult<Binary>`: 合约信息
pub fn query_cw721_contract_info(deps: Deps) -> StdResult<Binary> {
    let contract_info = CONTRACT_INFO.load(deps.storage)?;
    to_json_binary(&ContractInfoResponse {
        name: contract_info.name,
        symbol: contract_info.symbol,
    })
}
