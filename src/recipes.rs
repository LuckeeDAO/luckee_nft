//! 合成配方管理模块
//! 
//! 此模块包含合成配方的初始化和相关辅助函数
//! 定义了从基础 NFT 到高级 NFT 的合成路径

use cosmwasm_std::Storage;
use crate::error::ContractError;
use crate::state::RECIPES;
use crate::types::{NftKind, Recipe, RecipeInput};

// ========== 配方初始化函数 ==========

/// 初始化默认合成配方
/// 
/// 在合约部署时创建默认的合成配方，建立从基础到高级的合成链
/// 
/// # 参数
/// - `storage`: 存储接口
/// 
/// # 返回值
/// - `Result<(), ContractError>`: 初始化结果
pub fn initialize_default_recipes(storage: &mut dyn Storage) -> Result<(), ContractError> {
    // ========== 第一层：流萤合成配方 ==========
    // 需要 2 个四叶草合成 1 个流萤
    let firefly_recipe = Recipe {
        inputs: vec![RecipeInput { nft_kind: NftKind::Clover, count: 2 }],
        output: NftKind::Firefly,
        cost: None,
    };
    RECIPES.save(storage, NftKind::Firefly.to_key(), &firefly_recipe)?;

    // ========== 第二层：赤色锦鲤合成配方 ==========
    // 需要 2 个流萤合成 1 个赤色锦鲤
    let koi_recipe = Recipe {
        inputs: vec![RecipeInput { nft_kind: NftKind::Firefly, count: 2 }],
        output: NftKind::CrimsonKoi,
        cost: None,
    };
    RECIPES.save(storage, NftKind::CrimsonKoi.to_key(), &koi_recipe)?;

    // ========== 第三层：三愿神灯合成配方 ==========
    // 需要 5 个赤色锦鲤合成 1 个三愿神灯
    let lamp_recipe = Recipe {
        inputs: vec![RecipeInput { nft_kind: NftKind::CrimsonKoi, count: 5 }],
        output: NftKind::MagicalLamp,
        cost: None,
    };
    RECIPES.save(storage, NftKind::MagicalLamp.to_key(), &lamp_recipe)?;

    // ========== 第四层：命运纺锤合成配方 ==========
    // 需要 10 个三愿神灯合成 1 个命运纺锤
    let spindle_recipe = Recipe {
        inputs: vec![RecipeInput { nft_kind: NftKind::MagicalLamp, count: 10 }],
        output: NftKind::FatesSpindle,
        cost: None,
    };
    RECIPES.save(storage, NftKind::FatesSpindle.to_key(), &spindle_recipe)?;

    // ========== 第五层：悟道者合成配方 ==========
    // 需要 10 个命运纺锤合成 1 个悟道者
    let sage_recipe = Recipe {
        inputs: vec![RecipeInput { nft_kind: NftKind::FatesSpindle, count: 10 }],
        output: NftKind::Sage,
        cost: None,
    };
    RECIPES.save(storage, NftKind::Sage.to_key(), &sage_recipe)?;

    // ========== 第六层：紫薇帝星合成配方 ==========
    // 需要 10 个悟道者合成 1 个紫薇帝星
    let polaris_recipe = Recipe {
        inputs: vec![RecipeInput { nft_kind: NftKind::Sage, count: 10 }],
        output: NftKind::Polaris,
        cost: None,
    };
    RECIPES.save(storage, NftKind::Polaris.to_key(), &polaris_recipe)?;

    // ========== 第七层：轮盘之主合成配方 ==========
    // 需要 10 个紫薇帝星合成 1 个轮盘之主
    let roulette_recipe = Recipe {
        inputs: vec![RecipeInput { nft_kind: NftKind::Polaris, count: 10 }],
        output: NftKind::WheelOfDestiny,
        cost: None,
    };
    RECIPES.save(storage, NftKind::WheelOfDestiny.to_key(), &roulette_recipe)?;

    // ========== 第八层：造化元灵合成配方 ==========
    // 需要 10 个轮盘之主合成 1 个造化元灵（最高级）
    let genesis_recipe = Recipe {
        inputs: vec![RecipeInput { nft_kind: NftKind::WheelOfDestiny, count: 10 }],
        output: NftKind::Genesis,
        cost: None,
    };
    RECIPES.save(storage, NftKind::Genesis.to_key(), &genesis_recipe)?;

    Ok(())
}
