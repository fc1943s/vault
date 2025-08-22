param(
    $fast,
    $SkipSync,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../deps/alphabet/deps/polyglot/scripts/core.ps1


if (!$SkipSync) {
    Remove-Item ../dist -Recurse -Force -ErrorAction Ignore

    rsync -av `
        --exclude '.git' `
        --exclude '.history' `
        --exclude '.vscode' `
        --exclude 'file_cache' `
        --exclude 'target' `
        --exclude 'deps' `
        --exclude 'dist' `
    `
        --include 'LICENSE' `
        --include 'Dockerfile' `
        --include '*.editorconfig' `
        --include '*.edn' `
        --include '*.epub' `
        --include '*.gitignore' `
        --include '*.json' `
        --include '*.html' `
        --include '*.md' `
        --include '*.pdf' `
        --include '*.ps1' `
        --include '*.toml' `
    `
        --include '*/' `
        --exclude '*' `
        --no-links `
        --prune-empty-dirs `
        ../ `
        ../dist/

    Get-ChildItem -Path ../dist -Recurse -Force | Where-Object { $_.Name.StartsWith(".") } | ForEach-Object {
        Move-Item $_.FullName "$($_.FullName | Split-Path)/_$($_.Name)" -Force
    }
}

$vaultDir = (Resolve-Path ..).Path
$distDir = "$vaultDir/dist"

$targetDir = "$vaultDir/target/gh-pages"
New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
$targetDirLog = $targetDir
$targetDir = (Resolve-Path $targetDir).Path
$resolvedTargetDir = ResolveLink $targetDir
Write-Output "vault/scripts/publish.ps1 / targetDir: $targetDir / distDir: $distDir / targetDirLog: $targetDirLog / resolvedTargetDir: $resolvedTargetDir"
if (!(Test-Path $targetDir)) {
    exit 1
}
Set-Location "../target"
git clone $(git ls-remote --get-url origin) --branch gh-pages gh-pages
Set-Location $targetDir
git pull --rebase -Xtheirs
Set-Location $ScriptDir


{ . ../deps/alphabet/apps/documents/target/release/documents$(_exe) --source-dir $vaultDir --dist-dir $distDir --cache-dir $targetDir } | Invoke-Block

{ . ../deps/alphabet/deps/polyglot/apps/dir-tree-html/dist/DirTreeHtml$(_exe) --dir ../dist --html ../dist/index.html } | Invoke-Block
