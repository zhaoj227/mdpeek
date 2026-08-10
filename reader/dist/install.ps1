# Markdown 阅读器 一键安装脚本
# 功能：复制程序到用户目录、注册 .md 的「打开方式」入口。
$ErrorActionPreference = 'Stop'

$progid  = 'MDReader'
$dir     = Join-Path $env:LOCALAPPDATA 'Programs\mdreader'
$src     = Join-Path $PSScriptRoot 'mdreader.exe'
$exe     = Join-Path $dir 'mdreader.exe'

if (-not (Test-Path $src)) {
    Write-Host "[错误] 未找到 $src，请确认 install.bat 与 mdreader.exe 在同一目录。" -ForegroundColor Red
    exit 1
}

# 1. 复制程序
New-Item -ItemType Directory -Force -Path $dir | Out-Null
Copy-Item $src $exe -Force

# 2. 注册 ProgID
New-Item -Path "Registry::HKEY_CURRENT_USER\Software\Classes\$progid\shell\open\command" -Force | Out-Null
Set-Item -Path "Registry::HKEY_CURRENT_USER\Software\Classes\$progid" -Value 'Markdown 阅读器'
Set-Item -Path "Registry::HKEY_CURRENT_USER\Software\Classes\$progid\shell\open\command" -Value ('"{0}" "%1"' -f $exe)

# 3. 把本程序加入 .md 的「打开方式」列表（不改动 .md 默认关联）
New-Item -Path "Registry::HKEY_CURRENT_USER\Software\Classes\.md\OpenWithProgids" -Force | Out-Null
New-Item -Path "Registry::HKEY_CURRENT_USER\Software\Classes\.md\OpenWithProgids\$progid" -Force | Out-Null

Write-Host ''
Write-Host '安装完成！' -ForegroundColor Green
Write-Host '使用方法：在资源管理器中右键任意 .md 文件 → 打开方式 → Markdown 阅读器。'
Write-Host ''
