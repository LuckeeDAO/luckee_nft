//! 管理员功能实现模块
//! 
//! 此模块包含所有管理员和紧急控制功能，包括：
//! - 合约配置更新（铸造者、基础URI、外部合约等）
//! - 合约暂停和恢复
//! - 紧急资金提取
//! - 待处理状态清理

use cosmwasm_std::{DepsMut, MessageInfo, Response, Coin};

use crate::error::ContractError;
use crate::state::{CONFIG, CONTRACT_PAUSED};
use crate::helpers::check_contract_paused;

// ========== 管理员执行接口 ==========

/// 更新铸造者地址
/// 
/// 更改合约的铸造者地址，只有合约所有者可以执行此操作
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `new_minter`: 新的铸造者地址
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 更新结果
pub fn execute_update_minter(
    deps: DepsMut,
    info: MessageInfo,
    new_minter: String,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证所有者权限
    let mut config = CONFIG.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // 验证新铸造者地址格式并更新配置
    config.minter = deps.api.addr_validate(&new_minter)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_minter")
        .add_attribute("new_minter", new_minter))
}

/// 更新基础 URI
/// 
/// 更改合约的基础 URI，用于构建 NFT 的完整 URI
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `base_uri`: 新的基础 URI
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 更新结果
pub fn execute_update_base_uri(
    deps: DepsMut,
    info: MessageInfo,
    base_uri: String,
) -> Result<Response, ContractError> {
    // 检查合约是否暂停
    check_contract_paused(deps.storage)?;
    
    // 验证所有者权限
    let mut config = CONFIG.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // 更新基础 URI
    config.base_uri = Some(base_uri.clone());
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_base_uri")
        .add_attribute("base_uri", base_uri))
}



/// 暂停合约
/// 
/// 暂停合约的所有执行操作，只有合约所有者可以执行
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 暂停结果
pub fn execute_pause(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // 验证所有者权限
    let config = CONFIG.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // 设置合约为暂停状态
    CONTRACT_PAUSED.save(deps.storage, &true)?;

    Ok(Response::new()
        .add_attribute("action", "pause"))
}

/// 恢复合约
/// 
/// 恢复合约的正常执行，只有合约所有者可以执行
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 恢复结果
pub fn execute_unpause(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // 验证所有者权限
    let config = CONFIG.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // 设置合约为正常运行状态
    CONTRACT_PAUSED.save(deps.storage, &false)?;

    Ok(Response::new()
        .add_attribute("action", "unpause"))
}

/// 紧急提取资金
/// 
/// 紧急情况下提取合约中的资金，只有合约所有者可以执行
/// 
/// # 参数
/// - `deps`: 依赖对象，包含存储和API访问
/// - `info`: 消息信息，包含发送者
/// - `amount`: 要提取的资金列表
/// 
/// # 返回值
/// - `Result<Response, ContractError>`: 提取结果
pub fn execute_emergency_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    // 验证所有者权限
    let config = CONFIG.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // 构建银行转账消息
    let mut response = Response::new()
        .add_attribute("action", "emergency_withdraw");

    for coin in amount {
        let bank_msg = cosmwasm_std::BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![coin.clone()],
        };
        response = response.add_message(bank_msg);
    }

    Ok(response)
}

