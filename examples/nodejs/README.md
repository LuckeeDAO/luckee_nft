# Luckee NFT Node.js 示例

本目录包含 Luckee NFT 合约的 Node.js 集成示例，展示如何使用 `@cosmjs` 库与合约进行交互。

## 文件说明

- `luckee-nft-client.js` - 主要的客户端类，包含所有合约交互方法
- `test-examples.js` - 测试示例，展示各种功能的使用方法
- `package.json` - 项目依赖配置

## 安装依赖

```bash
npm install
```

## 配置

在使用示例之前，需要修改配置文件：

1. 在 `luckee-nft-client.js` 中修改 `CONFIG` 对象：
   - `contractAddress`: 替换为实际的合约地址
   - `mnemonic`: 替换为你的助记词

2. 在 `test-examples.js` 中修改 `TEST_CONFIG` 对象：
   - `contractAddress`: 替换为实际的合约地址
   - `mnemonic`: 替换为测试用的助记词

## 使用方法

### 1. 运行基本示例

```bash
npm start
```

这将运行 `luckee-nft-client.js` 中的示例代码，展示各种合约功能的使用方法。

### 2. 运行测试示例

```bash
npm test
```

这将运行 `test-examples.js` 中的测试代码，验证各种功能的正确性。

## 功能示例

### 铸造 NFT

```javascript
import { LuckeeNFTClient, NFT_KIND } from './luckee-nft-client.js';

const client = new LuckeeNFTClient(CONFIG);
await client.initialize();

// 铸造单个 NFT
await client.mintNFT(1, client.address, NFT_KIND.CLOVER, 'test_series');

// 批量铸造
const batchMints = [
  { tokenId: 2, owner: client.address, kind: NFT_KIND.CLOVER },
  { tokenId: 3, owner: client.address, kind: NFT_KIND.FIREFLY },
];
await client.batchMintNFTs(batchMints);
```

### 合成 NFT

```javascript
// 设置合成配方
const recipe = {
  inputs: [
    { kind: NFT_KIND.CLOVER, amount: 2 }
  ],
  cost: null
};
await client.setRecipe(NFT_KIND.FIREFLY, recipe);

// 预览合成
const preview = await client.previewSynthesis([2, 3], NFT_KIND.FIREFLY);

// 执行合成
await client.synthesize([2, 3], NFT_KIND.FIREFLY);
```

### 查询功能

```javascript
// 查询 NFT 所有者
const owner = await client.getOwner(1);

// 查询 NFT 信息
const nftInfo = await client.getNFTInfo(1);

// 查询用户拥有的 NFT
const userTokens = await client.getTokensByOwner(client.address);

// 查询按类型分类的 NFT
const cloverTokens = await client.getTokensByKind(NFT_KIND.CLOVER);
```

### 转移和销毁

```javascript
// 转移 NFT
await client.transferNFT(1, 'luckee1recipient...');

// 销毁 NFT
await client.burnNFT(1);
```

## 错误处理

所有方法都包含错误处理，失败时会抛出异常：

```javascript
try {
  await client.mintNFT(1, client.address, NFT_KIND.CLOVER);
  console.log('铸造成功');
} catch (error) {
  console.error('铸造失败:', error.message);
}
```

## 事件监听

合约操作会触发相应的事件，可以通过 Tendermint RPC 监听：

```javascript
// 监听铸造事件
const events = result.events;
const mintEvent = events.find(e => e.type === 'wasm-mint');
if (mintEvent) {
  const tokenId = mintEvent.attributes.find(a => a.key === 'token_id')?.value;
  console.log(`NFT 铸造成功: ${tokenId}`);
}
```

## 注意事项

1. **网络配置**: 确保使用正确的 RPC 端点和链 ID
2. **Gas 费用**: 操作需要支付 Gas 费用，确保账户有足够的代币
3. **权限**: 某些操作需要特定的权限（如铸造需要 minter 权限）
4. **测试环境**: 建议先在测试网上测试，确认无误后再在主网使用

## 支持的功能

- ✅ NFT 铸造（单个和批量）
- ✅ NFT 转移
- ✅ NFT 销毁
- ✅ NFT 合成
- ✅ 合成配方管理
- ✅ 各种查询功能
- ✅ 事件监听
- ✅ 错误处理

## 依赖库

- `@cosmjs/cosmwasm-stargate`: CosmWasm 合约交互
- `@cosmjs/proto-signing`: 签名和钱包管理
- `@cosmjs/stargate`: Cosmos SDK 功能

## 许可证

MIT License
