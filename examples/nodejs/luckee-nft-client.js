// Luckee NFT 合约 Node.js 集成示例
// 
// 本文件展示如何使用 SigningCosmWasmClient 调用 Luckee NFT 合约的各种功能
// 包括铸造、合成、转移、查询等操作

import { SigningCosmWasmClient, Secp256k1HdWallet } from '@cosmjs/cosmwasm-stargate';
import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing';
import { GasPrice } from '@cosmjs/stargate';

// 配置常量
const CONFIG = {
  // 网络配置
  rpcEndpoint: 'https://rpc.luckee-testnet.com:443',
  chainId: 'luckee-testnet',
  gasPrice: '0.025uluckee',
  
  // 合约地址
  contractAddress: 'luckee1contract...', // 替换为实际合约地址
  
  // 钱包配置
  mnemonic: 'your mnemonic phrase here...', // 替换为实际助记词
  keyName: 'admin',
};

// NFT 类型枚举
const NFT_KIND = {
  CLOVER: 'Clover',
  FIREFLY: 'Firefly',
  CRIMSON_KOI: 'CrimsonKoi',
  MAGICAL_LAMP: 'MagicalLamp',
  FATES_SPINDLE: 'FatesSpindle',
  SAGE: 'Sage',
  POLARIS: 'Polaris',
  WHEEL_OF_DESTINY: 'WheelOfDestiny',
  GENESIS: 'Genesis',
};

// 规模枚举
const SCALE = {
  TINY: 'Tiny',
  SMALL: 'Small',
  MEDIUM: 'Medium',
  LARGE: 'Large',
  HUGE: 'Huge',
};

class LuckeeNFTClient {
  constructor(config) {
    this.config = config;
    this.client = null;
    this.wallet = null;
    this.address = null;
  }

  // 初始化客户端
  async initialize() {
    try {
      // 创建钱包
      this.wallet = await DirectSecp256k1HdWallet.fromMnemonic(
        this.config.mnemonic,
        { prefix: 'luckee' }
      );

      // 获取地址
      const [account] = await this.wallet.getAccounts();
      this.address = account.address;

      // 创建客户端
      this.client = await SigningCosmWasmClient.connectWithSigner(
        this.config.rpcEndpoint,
        this.wallet,
        {
          gasPrice: GasPrice.fromString(this.config.gasPrice),
        }
      );

      console.log('✅ 客户端初始化成功');
      console.log(`地址: ${this.address}`);
      console.log(`链ID: ${this.config.chainId}`);
      console.log(`合约地址: ${this.config.contractAddress}`);

    } catch (error) {
      console.error('❌ 客户端初始化失败:', error);
      throw error;
    }
  }

  // 铸造 NFT
  async mintNFT(tokenId, owner, nftKind, seriesId = 'default') {
    try {
      const mintMsg = {
        mint: {
          token_id: tokenId,
          owner: owner,
          extension: {
            kind: nftKind,
            scale_origin: SCALE.TINY,
            physical_sku: null,
            crafted_from: null,
            series_id: seriesId,
            collection_group_id: null,
            serial_in_series: 1,
          },
        },
      };

      console.log(`🔄 铸造 NFT: ID=${tokenId}, Owner=${owner}, Kind=${nftKind}`);

      const result = await this.client.execute(
        this.address,
        this.config.contractAddress,
        mintMsg,
        'auto',
        '铸造 NFT'
      );

      console.log('✅ NFT 铸造成功');
      console.log(`交易哈希: ${result.transactionHash}`);
      console.log(`Gas 使用: ${result.gasUsed}`);

      return result;

    } catch (error) {
      console.error('❌ NFT 铸造失败:', error);
      throw error;
    }
  }

  // 批量铸造 NFT
  async batchMintNFTs(mints) {
    try {
      const batchMintMsg = {
        batch_mint: {
          mints: mints.map((mint, index) => ({
            token_id: mint.tokenId,
            owner: mint.owner,
            extension: {
              kind: mint.kind,
              scale_origin: SCALE.TINY,
              physical_sku: null,
              crafted_from: null,
              series_id: mint.seriesId || 'batch',
              collection_group_id: null,
              serial_in_series: index + 1,
            },
          })),
        },
      };

      console.log(`🔄 批量铸造 ${mints.length} 个 NFT`);

      const result = await this.client.execute(
        this.address,
        this.config.contractAddress,
        batchMintMsg,
        'auto',
        '批量铸造 NFT'
      );

      console.log('✅ 批量铸造成功');
      console.log(`交易哈希: ${result.transactionHash}`);

      return result;

    } catch (error) {
      console.error('❌ 批量铸造失败:', error);
      throw error;
    }
  }

  // 设置合成配方
  async setRecipe(target, recipe) {
    try {
      const setRecipeMsg = {
        set_recipe: {
          target: target,
          recipe: recipe,
        },
      };

      console.log(`🔄 设置合成配方: Target=${target}`);

      const result = await this.client.execute(
        this.address,
        this.config.contractAddress,
        setRecipeMsg,
        'auto',
        '设置合成配方'
      );

      console.log('✅ 合成配方设置成功');
      console.log(`交易哈希: ${result.transactionHash}`);

      return result;

    } catch (error) {
      console.error('❌ 合成配方设置失败:', error);
      throw error;
    }
  }

  // 执行合成
  async synthesize(inputs, target) {
    try {
      const synthesizeMsg = {
        synthesize: {
          inputs: inputs,
          target: target,
        },
      };

      console.log(`🔄 执行合成: Inputs=[${inputs.join(', ')}], Target=${target}`);

      const result = await this.client.execute(
        this.address,
        this.config.contractAddress,
        synthesizeMsg,
        'auto',
        '执行合成'
      );

      console.log('✅ 合成成功');
      console.log(`交易哈希: ${result.transactionHash}`);

      // 解析事件获取输出 token ID
      const events = result.events;
      const synthesizeEvent = events.find(e => e.type === 'wasm-synthesize');
      if (synthesizeEvent) {
        const outputTokenId = synthesizeEvent.attributes.find(a => a.key === 'output_token_id')?.value;
        console.log(`输出 NFT ID: ${outputTokenId}`);
      }

      return result;

    } catch (error) {
      console.error('❌ 合成失败:', error);
      throw error;
    }
  }

  // 转移 NFT
  async transferNFT(tokenId, recipient) {
    try {
      const transferMsg = {
        transfer_nft: {
          recipient: recipient,
          token_id: tokenId,
        },
      };

      console.log(`🔄 转移 NFT: ID=${tokenId}, To=${recipient}`);

      const result = await this.client.execute(
        this.address,
        this.config.contractAddress,
        transferMsg,
        'auto',
        '转移 NFT'
      );

      console.log('✅ NFT 转移成功');
      console.log(`交易哈希: ${result.transactionHash}`);

      return result;

    } catch (error) {
      console.error('❌ NFT 转移失败:', error);
      throw error;
    }
  }

  // 销毁 NFT
  async burnNFT(tokenId) {
    try {
      const burnMsg = {
        burn: {
          token_id: tokenId,
        },
      };

      console.log(`🔄 销毁 NFT: ID=${tokenId}`);

      const result = await this.client.execute(
        this.address,
        this.config.contractAddress,
        burnMsg,
        'auto',
        '销毁 NFT'
      );

      console.log('✅ NFT 销毁成功');
      console.log(`交易哈希: ${result.transactionHash}`);

      return result;

    } catch (error) {
      console.error('❌ NFT 销毁失败:', error);
      throw error;
    }
  }

  // 查询 NFT 所有者
  async getOwner(tokenId) {
    try {
      const queryMsg = {
        owner_of: {
          token_id: tokenId,
          include_expired: false,
        },
      };

      const result = await this.client.queryContractSmart(
        this.config.contractAddress,
        queryMsg
      );

      return result.owner;

    } catch (error) {
      console.error('❌ 查询所有者失败:', error);
      throw error;
    }
  }

  // 查询 NFT 信息
  async getNFTInfo(tokenId) {
    try {
      const queryMsg = {
        nft_info: {
          token_id: tokenId,
        },
      };

      const result = await this.client.queryContractSmart(
        this.config.contractAddress,
        queryMsg
      );

      return result;

    } catch (error) {
      console.error('❌ 查询 NFT 信息失败:', error);
      throw error;
    }
  }

  // 查询 NFT 元数据
  async getNFTMeta(tokenId) {
    try {
      const queryMsg = {
        token_meta: {
          token_id: tokenId,
        },
      };

      const result = await this.client.queryContractSmart(
        this.config.contractAddress,
        queryMsg
      );

      return result.meta;

    } catch (error) {
      console.error('❌ 查询 NFT 元数据失败:', error);
      throw error;
    }
  }

  // 查询用户拥有的 NFT
  async getTokensByOwner(owner, startAfter = null, limit = 30) {
    try {
      const queryMsg = {
        tokens: {
          owner: owner,
          start_after: startAfter,
          limit: limit,
        },
      };

      const result = await this.client.queryContractSmart(
        this.config.contractAddress,
        queryMsg
      );

      return result.tokens;

    } catch (error) {
      console.error('❌ 查询用户 NFT 失败:', error);
      throw error;
    }
  }

  // 查询按类型分类的 NFT
  async getTokensByKind(kind, startAfter = null, limit = 30) {
    try {
      const queryMsg = {
        tokens_by_kind: {
          kind: kind,
          start_after: startAfter,
          limit: limit,
        },
      };

      const result = await this.client.queryContractSmart(
        this.config.contractAddress,
        queryMsg
      );

      return result.tokens;

    } catch (error) {
      console.error('❌ 查询按类型分类的 NFT 失败:', error);
      throw error;
    }
  }

  // 查询合成配方
  async getRecipe(target) {
    try {
      const queryMsg = {
        recipe: {
          target: target,
        },
      };

      const result = await this.client.queryContractSmart(
        this.config.contractAddress,
        queryMsg
      );

      return result.recipe;

    } catch (error) {
      console.error('❌ 查询合成配方失败:', error);
      throw error;
    }
  }

  // 预览合成
  async previewSynthesis(inputs, target) {
    try {
      const queryMsg = {
        synthesis_preview: {
          inputs: inputs,
          target: target,
        },
      };

      const result = await this.client.queryContractSmart(
        this.config.contractAddress,
        queryMsg
      );

      return result;

    } catch (error) {
      console.error('❌ 预览合成失败:', error);
      throw error;
    }
  }

  // 查询合约信息
  async getContractInfo() {
    try {
      const queryMsg = {
        contract_info: {},
      };

      const result = await this.client.queryContractSmart(
        this.config.contractAddress,
        queryMsg
      );

      return result;

    } catch (error) {
      console.error('❌ 查询合约信息失败:', error);
      throw error;
    }
  }

  // 查询 Luckee 合约信息
  async getLuckeeContractInfo() {
    try {
      const queryMsg = {
        luckee_contract_info: {},
      };

      const result = await this.client.queryContractSmart(
        this.config.contractAddress,
        queryMsg
      );

      return result;

    } catch (error) {
      console.error('❌ 查询 Luckee 合约信息失败:', error);
      throw error;
    }
  }

  // 获取账户余额
  async getBalance() {
    try {
      const balance = await this.client.getBalance(this.address, 'uluckee');
      return balance;

    } catch (error) {
      console.error('❌ 查询余额失败:', error);
      throw error;
    }
  }
}

// 使用示例
async function main() {
  try {
    // 创建客户端实例
    const client = new LuckeeNFTClient(CONFIG);
    
    // 初始化客户端
    await client.initialize();

    // 查询账户余额
    const balance = await client.getBalance();
    console.log(`账户余额: ${balance.amount} ${balance.denom}`);

    // 查询合约信息
    const contractInfo = await client.getLuckeeContractInfo();
    console.log('合约信息:', contractInfo);

    // 示例1: 铸造 NFT
    console.log('\n=== 示例1: 铸造 NFT ===');
    await client.mintNFT(1, client.address, NFT_KIND.CLOVER, 'test_series');

    // 示例2: 批量铸造
    console.log('\n=== 示例2: 批量铸造 ===');
    const batchMints = [
      { tokenId: 2, owner: client.address, kind: NFT_KIND.CLOVER, seriesId: 'batch_series' },
      { tokenId: 3, owner: client.address, kind: NFT_KIND.CLOVER, seriesId: 'batch_series' },
    ];
    await client.batchMintNFTs(batchMints);

    // 示例3: 设置合成配方
    console.log('\n=== 示例3: 设置合成配方 ===');
    const recipe = {
      inputs: [
        {
          kind: NFT_KIND.CLOVER,
          amount: 2,
        },
      ],
      cost: null,
    };
    await client.setRecipe(NFT_KIND.FIREFLY, recipe);

    // 示例4: 预览合成
    console.log('\n=== 示例4: 预览合成 ===');
    const preview = await client.previewSynthesis([2, 3], NFT_KIND.FIREFLY);
    console.log('合成预览:', preview);

    // 示例5: 执行合成
    console.log('\n=== 示例5: 执行合成 ===');
    await client.synthesize([2, 3], NFT_KIND.FIREFLY);

    // 示例6: 查询用户拥有的 NFT
    console.log('\n=== 示例6: 查询用户 NFT ===');
    const userTokens = await client.getTokensByOwner(client.address);
    console.log('用户拥有的 NFT:', userTokens);

    // 示例7: 查询按类型分类的 NFT
    console.log('\n=== 示例7: 查询按类型分类的 NFT ===');
    const cloverTokens = await client.getTokensByKind(NFT_KIND.CLOVER);
    console.log('四叶草 NFT:', cloverTokens);

    // 示例8: 查询合成配方
    console.log('\n=== 示例8: 查询合成配方 ===');
    const fireflyRecipe = await client.getRecipe(NFT_KIND.FIREFLY);
    console.log('流萤合成配方:', fireflyRecipe);

  } catch (error) {
    console.error('❌ 示例执行失败:', error);
  }
}

// 如果直接运行此文件，执行示例
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch(console.error);
}

export { LuckeeNFTClient, NFT_KIND, SCALE };
