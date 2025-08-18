# PowerShell脚本以创建和发布新版本
Write-Host "开始创建并发布ShadowTLS v0.2.26..."

# 提交所有更改
git add .
git commit -m "更新版本至0.2.26：改进HMAC机制，减少检测风险"

# 创建新标签
git tag -a v0.2.26 -m "版本0.2.26：改进HMAC机制，减少检测风险"

# 推送更改和标签
git push origin dev
git push origin v0.2.26

Write-Host "完成！"
Write-Host "标签v0.2.26已推送至GitHub，自动构建过程应该已经开始。"
Write-Host "请访问 https://github.com/JCrun/shadow-tls/actions 查看构建进度。"
Write-Host "构建完成后，release将自动发布在 https://github.com/JCrun/shadow-tls/releases"
