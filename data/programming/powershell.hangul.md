
```프스1
```ps1
제트-시우지텡 -파트 . -헤쿠르시 -필리 | 에리-오브젝트 { $_.이스텐시옹 -이크 '.므드' } | 포레아시-오브젝트 {
Get-ChildItem -Path . -Recurse -File | Where-Object { $_.Extension -eq '.md' } | ForEach-Object {
$헬라치베파트 = $_.푸우나미.헤플라시((헤조우비-파트 .).파트 + '\', '')
$relativePath = $_.FullName.Replace((Resolve-Path .).Path + '\', '')
리치-오트푸트 $헬라치베파트
Write-Output $relativePath
}
}
```
```
