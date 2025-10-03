// Luckee NFT 合约测试示例
// 
// 本文件包含各种测试场景，展示如何测试合约功能

import { LuckeeNFTClient, NFT_KIND, SCALE } from './luckee-nft-client.js';

// 测试配置
const TEST_CONFIG = {
  rpcEndpoint: 'https://rpc.luckee-testnet.com:443',
  chainId: 'luckee-testnet',
  gasPrice: '0.025uluckee',
  contractAddress: 'luckee1contract...', // 替换为实际合约地址
  mnemonic: 'your test mnemonic phrase here...', // 替换为测试助记词
};

class LuckeeNFTTester {
  constructor(config) {
    this.client = new LuckeeNFTClient(config);
    this.testResults = [];
  }

  // 运行所有测试
  async runAllTests() {
    console.log('🧪 开始运行 Luckee NFT 合约测试...\n');

    try {
      await this.client.initialize();

      // 基础功能测试
      await this.testBasicQueries();
      await this.testMinting();
      await this.testBatchMinting();
      await this.testTransfers();
      await this.testBurning();

      // 合成功能测试
      await this.testRecipeManagement();
      await this.testSynthesis();

      // 边界条件测试
      await this.testEdgeCases();

      // 显示测试结果
      this.showTestResults();

    } catch (error) {
      console.error('❌ 测试执行失败:', error);
    }
  }

  // 记录测试结果
  recordTest(testName, success, error = null) {
    this.testResults.push({
      name: testName,
      success,
      error: error?.message || null,
      timestamp: new Date().toISOString(),
    });

    const status = success ? '✅' : '❌';
    console.log(`${status} ${testName}`);
    if (error) {
      console.log(`   错误: ${error.message}`);
    }
  }

  // 基础查询测试
  async testBasicQueries() {
    console.log('\n=== 基础查询测试 ===');

    try {
      // 测试合约信息查询
      const contractInfo = await this.client.getContractInfo();
      this.recordTest('查询合约信息', true);

      // 测试 Luckee 合约信息查询
      const luckeeInfo = await this.client.getLuckeeContractInfo();
      this.recordTest('查询 Luckee 合约信息', true);

      // 测试余额查询
      const balance = await this.client.getBalance();
      this.recordTest('查询账户余额', true);

    } catch (error) {
      this.recordTest('基础查询测试', false, error);
    }
  }

  // 铸造测试
  async testMinting() {
    console.log('\n=== 铸造测试 ===');

    try {
      // 测试单个铸造
      await this.client.mintNFT(1001, this.client.address, NFT_KIND.CLOVER, 'test_mint');
      this.recordTest('单个 NFT 铸造', true);

      // 验证铸造结果
      const owner = await this.client.getOwner(1001);
      if (owner === this.client.address) {
        this.recordTest('验证铸造所有权', true);
      } else {
        this.recordTest('验证铸造所有权', false, new Error('所有权不匹配'));
      }

      // 测试 NFT 信息查询
      const nftInfo = await this.client.getNFTInfo(1001);
      this.recordTest('查询 NFT 信息', true);

      // 测试 NFT 元数据查询
      const nftMeta = await this.client.getNFTMeta(1001);
      this.recordTest('查询 NFT 元数据', true);

    } catch (error) {
      this.recordTest('铸造测试', false, error);
    }
  }

  // 批量铸造测试
  async testBatchMinting() {
    console.log('\n=== 批量铸造测试 ===');

    try {
      const batchMints = [
        { tokenId: 2001, owner: this.client.address, kind: NFT_KIND.CLOVER, seriesId: 'batch_test' },
        { tokenId: 2002, owner: this.client.address, kind: NFT_KIND.FIREFLY, seriesId: 'batch_test' },
        { tokenId: 2003, owner: this.client.address, kind: NFT_KIND.CLOVER, seriesId: 'batch_test' },
      ];

      await this.client.batchMintNFTs(batchMints);
      this.recordTest('批量铸造 NFT', true);

      // 验证批量铸造结果
      for (const mint of batchMints) {
        const owner = await this.client.getOwner(mint.tokenId);
        if (owner !== this.client.address) {
          this.recordTest(`验证批量铸造 ${mint.tokenId}`, false, new Error('所有权不匹配'));
          return;
        }
      }
      this.recordTest('验证批量铸造结果', true);

    } catch (error) {
      this.recordTest('批量铸造测试', false, error);
    }
  }

  // 转移测试
  async testTransfers() {
    console.log('\n=== 转移测试 ===');

    try {
      // 创建测试接收者地址（使用不同的助记词）
      const testRecipient = 'luckee1testrecipient...'; // 替换为实际测试地址

      // 测试转移
      await this.client.transferNFT(1001, testRecipient);
      this.recordTest('NFT 转移', true);

      // 验证转移结果
      const newOwner = await this.client.getOwner(1001);
      if (newOwner === testRecipient) {
        this.recordTest('验证转移结果', true);
      } else {
        this.recordTest('验证转移结果', false, new Error('新所有者不匹配'));
      }

      // 转移回原地址
      await this.client.transferNFT(1001, this.client.address);
      this.recordTest('NFT 转移回原地址', true);

    } catch (error) {
      this.recordTest('转移测试', false, error);
    }
  }

  // 销毁测试
  async testBurning() {
    console.log('\n=== 销毁测试 ===');

    try {
      // 测试销毁
      await this.client.burnNFT(2003);
      this.recordTest('NFT 销毁', true);

      // 验证销毁结果（查询应该失败）
      try {
        await this.client.getOwner(2003);
        this.recordTest('验证销毁结果', false, new Error('NFT 应该已被销毁'));
      } catch (error) {
        if (error.message.includes('Token not found')) {
          this.recordTest('验证销毁结果', true);
        } else {
          this.recordTest('验证销毁结果', false, error);
        }
      }

    } catch (error) {
      this.recordTest('销毁测试', false, error);
    }
  }

  // 配方管理测试
  async testRecipeManagement() {
    console.log('\n=== 配方管理测试 ===');

    try {
      // 测试设置合成配方
      const recipe = {
        inputs: [
          {
            kind: NFT_KIND.CLOVER,
            amount: 2,
          },
        ],
        cost: null,
      };

      await this.client.setRecipe(NFT_KIND.FIREFLY, recipe);
      this.recordTest('设置合成配方', true);

      // 测试查询合成配方
      const retrievedRecipe = await this.client.getRecipe(NFT_KIND.FIREFLY);
      if (retrievedRecipe && retrievedRecipe.inputs.length === 1) {
        this.recordTest('查询合成配方', true);
      } else {
        this.recordTest('查询合成配方', false, new Error('配方不匹配'));
      }

    } catch (error) {
      this.recordTest('配方管理测试', false, error);
    }
  }

  // 合成测试
  async testSynthesis() {
    console.log('\n=== 合成测试 ===');

    try {
      // 测试合成预览
      const preview = await this.client.previewSynthesis([2001, 2002], NFT_KIND.FIREFLY);
      if (preview.can_synthesize) {
        this.recordTest('合成预览', true);
      } else {
        this.recordTest('合成预览', false, new Error('无法合成'));
      }

      // 测试执行合成
      await this.client.synthesize([2001, 2002], NFT_KIND.FIREFLY);
      this.recordTest('执行合成', true);

      // 验证合成结果（输入 NFT 应该被销毁）
      try {
        await this.client.getOwner(2001);
        this.recordTest('验证合成结果 - 输入1', false, new Error('输入 NFT 应该被销毁'));
      } catch (error) {
        if (error.message.includes('Token not found')) {
          this.recordTest('验证合成结果 - 输入1', true);
        } else {
          this.recordTest('验证合成结果 - 输入1', false, error);
        }
      }

      try {
        await this.client.getOwner(2002);
        this.recordTest('验证合成结果 - 输入2', false, new Error('输入 NFT 应该被销毁'));
      } catch (error) {
        if (error.message.includes('Token not found')) {
          this.recordTest('验证合成结果 - 输入2', true);
        } else {
          this.recordTest('验证合成结果 - 输入2', false, error);
        }
      }

    } catch (error) {
      this.recordTest('合成测试', false, error);
    }
  }

  // 边界条件测试
  async testEdgeCases() {
    console.log('\n=== 边界条件测试 ===');

    try {
      // 测试查询不存在的 NFT
      try {
        await this.client.getOwner(9999);
        this.recordTest('查询不存在的 NFT', false, new Error('应该返回错误'));
      } catch (error) {
        if (error.message.includes('Token not found')) {
          this.recordTest('查询不存在的 NFT', true);
        } else {
          this.recordTest('查询不存在的 NFT', false, error);
        }
      }

      // 测试查询不存在的合成配方
      try {
        await this.client.getRecipe(NFT_KIND.GENESIS);
        this.recordTest('查询不存在的合成配方', false, new Error('应该返回空结果'));
      } catch (error) {
        this.recordTest('查询不存在的合成配方', true);
      }

      // 测试按类型查询
      const cloverTokens = await this.client.getTokensByKind(NFT_KIND.CLOVER);
      this.recordTest('按类型查询 NFT', true);

      // 测试按系列查询
      const seriesTokens = await this.client.getTokensByOwner(this.client.address);
      this.recordTest('按所有者查询 NFT', true);

    } catch (error) {
      this.recordTest('边界条件测试', false, error);
    }
  }

  // 显示测试结果
  showTestResults() {
    console.log('\n=== 测试结果汇总 ===');
    
    const totalTests = this.testResults.length;
    const passedTests = this.testResults.filter(r => r.success).length;
    const failedTests = totalTests - passedTests;

    console.log(`总测试数: ${totalTests}`);
    console.log(`通过: ${passedTests}`);
    console.log(`失败: ${failedTests}`);
    console.log(`成功率: ${((passedTests / totalTests) * 100).toFixed(2)}%`);

    if (failedTests > 0) {
      console.log('\n失败的测试:');
      this.testResults
        .filter(r => !r.success)
        .forEach(r => {
          console.log(`  ❌ ${r.name}: ${r.error}`);
        });
    }

    console.log('\n测试完成！');
  }
}

// 运行测试
async function runTests() {
  const tester = new LuckeeNFTTester(TEST_CONFIG);
  await tester.runAllTests();
}

// 如果直接运行此文件，执行测试
if (import.meta.url === `file://${process.argv[1]}`) {
  runTests().catch(console.error);
}

export { LuckeeNFTTester };
