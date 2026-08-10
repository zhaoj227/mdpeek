# Markdown 阅读器 卸载脚本
$progid = 'MDReader'

Remove-Item -Path "Registry::HKEY_CURRENT_USER\Software\Classes\.md\OpenWithProgids\$progid" -ErrorAction SilentlyContinue
Remove-Item -Path "Registry::HKEY_CURRENT_USER\Software\Classes\$progid" -Recurse -ErrorAction SilentlyContinue

$dir = Join-Path $env:LOCALAPPDATA 'Programs\mdreader'
Remove-Item -Path $dir -Recurse -Force -ErrorAction SilentlyContinue

Write-Host '已卸载。' -ForegroundColor Green
