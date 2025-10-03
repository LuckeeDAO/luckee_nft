//! Luckee NFT 合约主入口文件
//! 
//! 此文件包含合约的所有入口点（instantiate/execute/query/migrate）
//! 以及主调度逻辑。具体的功能实现都委托到各个子模块中。

use cosmwasm_std::{
    entry_point, to_json_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Binary,
};
use cw2::{set_contract_version, get_contract_version};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    Config, CONFIG, TOTAL_SUPPLY, STORAGE_VERSION, CONTRACT_PAUSED,
    CONTRACT_INFO, ContractInfo, TOKEN_META, NEXT_TOKEN_ID,
};

// 导入各个功能模块
use crate::cw721::*;      // 标准 CW721 接口实现
use crate::luckee::*;     // Luckee 扩展功能（合成、铸造等）
use crate::admin::*;      // 管理员功能（暂停、紧急提取等）
use crate::recipes::*;    // 配方管理

// 合约基本信息
const CONTRACT_NAME: &str = "crates.io:luckee_nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


/// 合约初始化入口点
/// 
/// 在合约部署时调用，用于设置初始配置和状态
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `_env`: 环境信息（未使用）
/// - `info`: 消息信息，包含发送者等
/// - `msg`: 初始化消息，包含合约配置
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 初始化结果
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // 设置合约版本信息
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // 创建合约配置
    let config = Config {
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        minter: deps.api.addr_validate(&msg.minter)?,
        base_uri: msg.base_uri.clone(),
        owner: info.sender.clone(),
    };

    // 保存配置和初始状态
    CONFIG.save(deps.storage, &config)?;
    TOTAL_SUPPLY.save(deps.storage, &0u64)?;
    NEXT_TOKEN_ID.save(deps.storage, &1u64)?;
    
    // 初始化存储版本
    STORAGE_VERSION.save(deps.storage, &CONTRACT_VERSION.to_string())?;
    
    // 初始化合约状态为未暂停
    CONTRACT_PAUSED.save(deps.storage, &false)?;
    
    // 初始化 CW721 标准合约信息
    let contract_info = ContractInfo {
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
    };
    CONTRACT_INFO.save(deps.storage, &contract_info)?;

    // 初始化默认合成配方
    initialize_default_recipes(deps.storage)?;

    // 返回初始化成功的响应
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("name", msg.name)
        .add_attribute("symbol", msg.symbol)
        .add_attribute("minter", msg.minter))
}

/// 合约执行入口点
/// 
/// 处理所有执行消息，根据消息类型路由到相应的处理函数
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `env`: 环境信息，包含区块高度、时间等
/// - `info`: 消息信息，包含发送者和资金
/// - `msg`: 执行消息，包含具体的操作类型和参数
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 执行结果
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // ========== 标准 CW721 接口 ==========
        ExecuteMsg::TransferNft { recipient, token_id } => {
            // 转移 NFT 所有权
            execute_transfer_nft(deps, env, info, recipient, token_id)
        }
        ExecuteMsg::Approve { spender, token_id, expires } => {
            // 批准特定地址操作特定 NFT
            execute_approve(deps, info, spender, token_id, expires)
        }
        ExecuteMsg::Revoke { spender, token_id } => {
            // 撤销特定地址对特定 NFT 的批准
            execute_revoke(deps, info, spender, token_id)
        }
        ExecuteMsg::ApproveAll { operator, expires } => {
            // 批准操作员管理所有 NFT
            execute_approve_all(deps, info, operator, expires)
        }
        ExecuteMsg::RevokeAll { operator } => {
            // 撤销操作员对所有 NFT 的管理权限
            execute_revoke_all(deps, info, operator)
        }

        // ========== Luckee 扩展接口 ==========
        ExecuteMsg::Mint { token_id, owner, extension } => {
            // 铸造新的 NFT
            execute_mint(deps, info, token_id, owner, extension)
        }
        ExecuteMsg::Burn { token_id } => {
            // 销毁 NFT
            execute_burn(deps, info, token_id)
        }

        // ========== 管理员接口 ==========
        ExecuteMsg::UpdateMinter { new_minter } => {
            // 更新铸造者地址
            execute_update_minter(deps, info, new_minter)
        }
        ExecuteMsg::UpdateBaseUri { base_uri } => {
            // 更新基础 URI
            execute_update_base_uri(deps, info, base_uri)
        }

        // ========== 合成相关接口 ==========
        ExecuteMsg::SetRecipe { target, recipe } => {
            // 设置合成配方
            execute_set_recipe(deps, info, target, recipe)
        }
        ExecuteMsg::RemoveRecipe { target } => {
            // 删除合成配方
            execute_remove_recipe(deps, info, target)
        }
        ExecuteMsg::Synthesize { inputs, target } => {
            // 执行合成操作
            execute_synthesize(deps, env, info, inputs, target)
        }

        // ========== 批量操作接口 ==========
        ExecuteMsg::BatchMint { mints } => {
            // 批量铸造 NFT
            execute_batch_mint(deps, info, mints)
        }
        ExecuteMsg::SetMinter { minter, allowed } => {
            // 设置铸造者权限
            execute_set_minter(deps, info, minter, allowed)
        }
        
        
        // ========== 访问控制和紧急机制 ==========
        ExecuteMsg::Pause {} => {
            // 暂停合约
            execute_pause(deps, info)
        }
        ExecuteMsg::Unpause {} => {
            // 恢复合约
            execute_unpause(deps, info)
        }
        ExecuteMsg::EmergencyWithdraw { amount } => {
            // 紧急提取资金
            execute_emergency_withdraw(deps, info, amount)
        }
        
    }
}

/// 合约查询入口点
/// 
/// 处理所有查询消息，根据消息类型返回相应的数据
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `env`: 环境信息，包含区块高度、时间等
/// - `msg`: 查询消息，包含具体的查询类型和参数
/// 
/// # 返回值
/// - `StdResult<Binary>`: 查询结果，以二进制格式返回
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // ========== 标准 CW721 查询 ==========
        QueryMsg::OwnerOf { token_id, include_expired } => {
            // 查询 NFT 的所有者信息
            query_owner_of(deps, env, token_id, include_expired)
        }
        QueryMsg::NftInfo { token_id } => {
            // 查询 NFT 的详细信息
            query_nft_info(deps, token_id)
        }
        QueryMsg::Approvals { token_id, include_expired } => {
            // 查询 NFT 的批准信息
            query_approvals(deps, env, token_id, include_expired)
        }
        QueryMsg::IsApprovedForAll { owner, operator } => {
            // 查询操作员是否被批准管理所有 NFT
            query_is_approved_for_all(deps, env, owner, operator)
        }
        QueryMsg::TokenUri { token_id } => {
            // 查询 NFT 的 URI 信息
            query_token_uri(deps, token_id)
        }
        QueryMsg::AllTokens { start_after, limit } => {
            // 查询所有 NFT 列表
            query_all_tokens(deps, env, start_after, limit)
        }
        QueryMsg::Tokens { owner, start_after, limit } => {
            // 查询指定用户拥有的 NFT 列表
            query_tokens(deps, env, owner, start_after, limit)
        }

        // ========== Luckee 扩展查询 ==========
        QueryMsg::TokenMeta { token_id } => {
            // 查询 NFT 的扩展元数据
            let meta = TOKEN_META.load(deps.storage, token_id)?;
            to_json_binary(&crate::msg::TokenMetaResponse { meta })
        }
        QueryMsg::TokensByKind { kind, start_after, limit } => {
            // 按类型查询 NFT 列表
            query_tokens_by_kind(deps, kind, start_after, limit)
        }
        QueryMsg::TokensBySeries { series_id, start_after, limit } => {
            // 按系列查询 NFT 列表
            query_tokens_by_series(deps, series_id, start_after, limit)
        }
        QueryMsg::TokensByGroup { group_id, start_after, limit } => {
            // 按组查询 NFT 列表
            query_tokens_by_group(deps, group_id, start_after, limit)
        }
        QueryMsg::LuckeeContractInfo {} => {
            // 查询 Luckee 合约信息
            query_contract_info(deps)
        }

        // ========== 合成相关查询 ==========
        QueryMsg::Recipe { target } => {
            // 查询指定目标的合成配方
            let recipe = crate::state::RECIPES.may_load(deps.storage, target.to_key())?;
            to_json_binary(&crate::msg::RecipeResponse { recipe })
        }
        QueryMsg::AllRecipes { start_after, limit } => {
            // 查询所有合成配方
            query_all_recipes(deps, start_after, limit)
        }
        QueryMsg::SynthesisPreview { inputs, target } => {
            // 预览合成操作的结果
            query_synthesis_preview(deps, inputs, target)
        }
        
        // ========== CW721 集成查询 ==========
        QueryMsg::GetNftContract {} => {
            // 查询外部 CW721 合约地址
            query_nft_contract(deps)
        }
        
        // ========== 标准 CW721 合约信息查询 ==========
        QueryMsg::ContractInfo {} => {
            // 查询标准 CW721 合约信息
            query_cw721_contract_info(deps)
        }
    }
}


/// 合约迁移入口点
/// 
/// 处理合约升级时的数据迁移和版本更新
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `_env`: 环境信息（未使用）
/// - `_msg`: 迁移消息（未使用）
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 迁移结果
#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: cosmwasm_std::Empty) -> Result<Response, ContractError> {
    // 获取当前合约版本
    let current_version = get_contract_version(deps.storage)?;
    
    // 更新合约版本信息
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    // 更新存储版本
    STORAGE_VERSION.save(deps.storage, &CONTRACT_VERSION.to_string())?;
    
    // 确保暂停状态被正确初始化（向后兼容性处理）
    if CONTRACT_PAUSED.may_load(deps.storage)?.is_none() {
        CONTRACT_PAUSED.save(deps.storage, &false)?;
    }

    // 返回迁移成功的响应
    Ok(Response::new()
        .add_attribute("method", "migrate")
        .add_attribute("previous_version", current_version.version)
        .add_attribute("new_version", CONTRACT_VERSION))
}