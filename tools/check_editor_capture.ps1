param(
    [Parameter(Mandatory = $true)]
    [string]$Path
)

# Runtime regression check: capture the open editor at 1280x720, GUI scale 3.
# Its left header is opaque dark; opaque label glyphs must reach this region.
# Before the November 18 ARGB adaptation this region contained zero bright pixels.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$image = [System.Drawing.Bitmap]::FromFile((Resolve-Path $Path))
try {
    if ($image.Width -ne 1280 -or $image.Height -ne 720) {
        throw 'Expected a 1280x720 editor capture with GUI scale 3.'
    }
    $bright = 0
    for ($y = 5; $y -lt 55; $y++) {
        for ($x = 10; $x -lt 400; $x++) {
            $pixel = $image.GetPixel($x, $y)
            if ($pixel.R -gt 200 -and $pixel.G -gt 200 -and $pixel.B -gt 200) {
                $bright++
            }
        }
    }
    if ($bright -le 50) {
        throw "Editor labels are invisible: only $bright bright header pixels."
    }
    Write-Output $bright
} finally {
    $image.Dispose()
}
