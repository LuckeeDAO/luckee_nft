#!/bin/bash

# 紧急修复脚本 - 直接修改系统环境
# 解决 GVM_DEBUG unbound variable 错误

echo "🚨 执行紧急修复..."

# 创建临时环境变量文件
cat > /tmp/gvm_env_fix.sh << 'EOF'
#!/bin/bash
export GVM_DEBUG=0
export GVM_ROOT=/home/lc/.gvm
EOF

# 在系统级设置环境变量
echo "GVM_DEBUG=0" | sudo tee -a /etc/environment 2>/dev/null || echo "无法写入 /etc/environment"

# 创建用户级环境变量文件
cat > /home/lc/.gvm_env << 'EOF'
#!/bin/bash
# GVM 环境变量修复
export GVM_DEBUG=0
export GVM_ROOT=/home/lc/.gvm
EOF

# 修改 bashrc 以优先加载环境变量
cp /home/lc/.bashrc /home/lc/.bashrc.backup.emergency.$(date +%Y%m%d_%H%M%S)

# 在 bashrc 开头添加环境变量设置
cat > /tmp/bashrc_fix << 'EOF'
# 紧急修复：设置 GVM_DEBUG 环境变量
export GVM_DEBUG=0

EOF

# 合并文件
cat /tmp/bashrc_fix /home/lc/.bashrc.backup.emergency.$(date +%Y%m%d_%H%M%S) > /home/lc/.bashrc

echo "✅ 紧急修复完成"
echo "请重新启动终端或执行: source ~/.bashrc"
