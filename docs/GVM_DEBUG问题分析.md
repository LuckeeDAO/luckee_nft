# GVM_DEBUG 问题分析报告

## 问题描述

在运行任何命令时都会报错：
```
environment: line 2935: GVM_DEBUG: unbound variable
```

## 根本原因

GVM (Go Version Manager) 在 `.bashrc` 中被加载，但其脚本在 `set -u` 模式下访问未定义的 `GVM_DEBUG` 变量。

### 问题定位

1. **GVM 加载位置**: `~/.bashrc` 第 156 行
   ```bash
   [[ -s "/home/lc/.gvm/scripts/gvm" ]] && source "/home/lc/.gvm/scripts/gvm"
   ```

2. **错误来源**: `/home/lc/.gvm/scripts/env/cd` 第 63 行
   ```bash
   [[ "${GVM_DEBUG}" -eq 1 ]] && echo "Resolving defaults..."
   ```

3. **触发条件**: 
   - 当 shell 以 `set -u` 模式运行时（禁止未定义变量）
   - GVM 脚本尝试访问 `GVM_DEBUG` 变量
   - 该变量未被初始化

## 解决方案

### 方案 1: 修复 GVM 脚本（推荐）

在 GVM 脚本中添加默认值检查：

```bash
# 在 ~/.gvm/scripts/env/cd 中修改第 63 行
[[ "${GVM_DEBUG:-0}" -eq 1 ]] && echo "Resolving defaults..."
```

使用 `${GVM_DEBUG:-0}` 语法，如果变量未定义则默认为 0。

### 方案 2: 在 .bashrc 中初始化变量

在加载 GVM 之前添加：

```bash
# 在 ~/.bashrc 第 156 行之前添加
export GVM_DEBUG=0

[[ -s "/home/lc/.gvm/scripts/gvm" ]] && source "/home/lc/.gvm/scripts/gvm"
```

### 方案 3: 使用条件加载

修改 .bashrc 中的 GVM 加载逻辑：

```bash
if [[ -s "/home/lc/.gvm/scripts/gvm" ]]; then
    export GVM_DEBUG=${GVM_DEBUG:-0}
    source "/home/lc/.gvm/scripts/gvm"
fi
```

### 方案 4: 临时绕过（用于命令执行）

在命令前设置变量：

```bash
GVM_DEBUG=0 git status
GVM_DEBUG=0 cargo check
```

## 影响范围

GVM 脚本中使用 GVM_DEBUG 的位置共 55 处：
- `/home/lc/.gvm/scripts/env/use`
- `/home/lc/.gvm/scripts/env/cd`
- 其他多个脚本文件

## 立即修复脚本

创建自动修复脚本来解决此问题。

## 验证修复

修复后运行以下命令验证：

```bash
bash -c 'set -u; source ~/.bashrc'
git status
cargo check
```

应该不再出现 unbound variable 错误。
