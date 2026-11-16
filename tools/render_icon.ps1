# Renders images/icon.png (256x256) from the geometry of images/icon.svg:
# the Marketplace wants a PNG for the extension icon.
#
# Usage: pwsh tools/render_icon.ps1

Add-Type -AssemblyName System.Drawing

$size = 256
$scale = $size / 128.0
$out = Join-Path $PSScriptRoot "..\images\icon.png"

$bitmap = New-Object System.Drawing.Bitmap $size, $size
$g = [System.Drawing.Graphics]::FromImage($bitmap)
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
$g.ScaleTransform($scale, $scale)
$g.Clear([System.Drawing.Color]::Transparent)

function Color($hex) { [System.Drawing.ColorTranslator]::FromHtml($hex) }

# The rounded background, its gradient from the top left.
$path = New-Object System.Drawing.Drawing2D.GraphicsPath
$r = 26; $d = 2 * $r; $x = 4; $y = 4; $w = 120; $h = 120
$path.AddArc($x, $y, $d, $d, 180, 90)
$path.AddArc($x + $w - $d, $y, $d, $d, 270, 90)
$path.AddArc($x + $w - $d, $y + $h - $d, $d, $d, 0, 90)
$path.AddArc($x, $y + $h - $d, $d, $d, 90, 90)
$path.CloseFigure()
$gradient = New-Object System.Drawing.Drawing2D.LinearGradientBrush `
    (New-Object System.Drawing.PointF 4, 4), (New-Object System.Drawing.PointF 124, 124), `
    (Color "#2fa37a"), (Color "#1b5e4b")
$g.FillPath($gradient, $path)

function Pen($hex, $width) {
    $pen = New-Object System.Drawing.Pen (Color $hex), $width
    $pen.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
    $pen.EndCap = [System.Drawing.Drawing2D.LineCap]::Round
    $pen
}

# The X.
$white = Pen "#ffffff" 16
$g.DrawLine($white, 34, 32, 80, 90)
$g.DrawLine($white, 80, 32, 34, 90)

# The lens.
$lens = New-Object System.Drawing.SolidBrush (Color "#1b5e4b")
$g.FillEllipse($lens, 71, 69, 34, 34)
$g.DrawEllipse((Pen "#ffd166" 7), 71, 69, 34, 34)
$g.DrawLine((Pen "#ffd166" 9), 100, 98, 112, 110)

$bitmap.Save($out, [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose(); $bitmap.Dispose()
Write-Output "wrote $out"
