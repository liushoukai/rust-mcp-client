#!/bin/bash

# MCP Server 测试脚本

echo "开始测试 MCP Server..."
echo ""

# 构建项目
echo "1. 构建项目..."
cargo build
echo ""

# 测试1: initialize
echo "2. 测试 initialize..."
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}' | cargo run 2>/dev/null | tail -1 | jq .
echo ""

# 测试2: tools/list
echo "3. 测试 tools/list..."
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | cargo run 2>/dev/null | tail -1 | jq .
echo ""

# 测试3: tools/call
echo "4. 测试 tools/call (get_ip_info)..."
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_ip_info"}}' | cargo run 2>/dev/null | tail -1 | jq .
echo ""

echo "测试完成！"
