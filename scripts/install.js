#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const { promisify } = require('util');
const { pipeline } = require('stream');

const streamPipeline = promisify(pipeline);

// 读取包版本
const packageJson = require('../package.json');
const version = packageJson.version;

// 检测平台和架构
const platform = process.platform;
const arch = process.arch;

// 二进制文件名映射
const binMap = {
  'darwin-x64': 'rust-mcp-client-darwin-x64',
  'darwin-arm64': 'rust-mcp-client-darwin-arm64',
  'linux-x64': 'rust-mcp-client-linux-x64',
  'linux-arm64': 'rust-mcp-client-linux-arm64',
  'win32-x64': 'rust-mcp-client-win32-x64.exe',
};

const binKey = `${platform}-${arch}`;
const binName = binMap[binKey];

if (!binName) {
  console.error(`错误: 不支持的平台 ${platform}-${arch}`);
  console.error('支持的平台:', Object.keys(binMap).join(', '));
  process.exit(1);
}

// 设置路径
const binDir = path.join(__dirname, '../bin');
const binPath = path.join(binDir, binName);

// GitHub Release URL
// 修改为你的 GitHub 用户名和仓库名
const GITHUB_USER = 'liushoukai';
const GITHUB_REPO = 'rust-mcp-client';
const downloadUrl = `https://github.com/${GITHUB_USER}/${GITHUB_REPO}/releases/download/v${version}/${binName}`;

console.log(`正在为 ${platform}-${arch} 下载二进制文件...`);
console.log(`版本: v${version}`);
console.log(`下载地址: ${downloadUrl}`);

// 创建 bin 目录
if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir, { recursive: true });
}

// 下载文件
async function download(url, dest) {
  return new Promise((resolve, reject) => {
    https.get(url, {
      headers: {
        'User-Agent': 'npm-installer'
      }
    }, (response) => {
      // 处理重定向
      if (response.statusCode === 301 || response.statusCode === 302) {
        return download(response.headers.location, dest)
          .then(resolve)
          .catch(reject);
      }

      if (response.statusCode !== 200) {
        reject(new Error(`下载失败: HTTP ${response.statusCode}`));
        return;
      }

      const file = fs.createWriteStream(dest);
      const totalSize = parseInt(response.headers['content-length'], 10);
      let downloadedSize = 0;
      let lastPercent = 0;

      response.on('data', (chunk) => {
        downloadedSize += chunk.length;
        const percent = Math.floor((downloadedSize / totalSize) * 100);

        // 每 10% 输出一次进度
        if (percent >= lastPercent + 10) {
          process.stdout.write(`\r下载进度: ${percent}%`);
          lastPercent = percent;
        }
      });

      streamPipeline(response, file)
        .then(() => {
          console.log('\r下载进度: 100%');
          console.log('下载完成!');

          // 在 Unix 系统上添加执行权限
          if (platform !== 'win32') {
            fs.chmodSync(dest, '755');
          }

          resolve();
        })
        .catch(reject);
    }).on('error', reject);
  });
}

// 执行下载
download(downloadUrl, binPath)
  .then(() => {
    console.log(`\n安装成功! 二进制文件位于: ${binPath}`);
    console.log('\n使用方法:');
    console.log('  npx @liushoukai/rust-mcp-client');
    console.log('  或在 MCP 配置中使用:');
    console.log('  { "command": "npx", "args": ["-y", "@liushoukai/rust-mcp-client"] }');
  })
  .catch((err) => {
    console.error('\n安装失败:', err.message);
    console.error('\n可能的原因:');
    console.error('1. 网络连接问题');
    console.error('2. GitHub Release 尚未发布');
    console.error(`3. 版本 v${version} 的二进制文件不存在`);
    console.error('\n手动下载地址:');
    console.error(downloadUrl);

    // 清理失败的文件
    if (fs.existsSync(binPath)) {
      fs.unlinkSync(binPath);
    }

    process.exit(1);
  });
