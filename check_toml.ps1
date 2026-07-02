$bytes = [System.IO.File]::ReadAllBytes('C:\Users\spenc\dev\savant-trading\config\test-anvil.toml')
if ($bytes[0] -eq 0xEF -and $bytes[1] -eq 0xBB -and $bytes[2] -eq 0xBF) {
    Write-Host "BOM detected, removing..."
    $newBytes = $bytes[3..($bytes.Length - 1)]
    [System.IO.File]::WriteAllBytes('C:\Users\spenc\dev\savant-trading\config\test-anvil.toml', $newBytes)
    Write-Host "BOM removed. New size: $($newBytes.Length) bytes"
} else {
    Write-Host "No BOM detected"
}
