package librt

import (
	"image"
	"image/color"
)

/* Source code inspired from https://github.com/i-am-g2/Tr/blob/master/tr/vector.go
   Author: i-am-g2
   License: NoLicense
*/
func WriteColor(X, Y, samplePerPixel int, pixelColor *Color, image *image.RGBA) {
	scale := 1.0 / float64(samplePerPixel)
	R := uint8(256 * Clamp(pixelColor.X()*scale, 0, 0.999))
	G := uint8(256 * Clamp(pixelColor.Y()*scale, 0, 0.999))
	B := uint8(256 * Clamp(pixelColor.Z()*scale, 0, 0.999))
	color := color.RGBA{R, G, B, 255}

	image.Set(X, image.Bounds().Dy()-Y-1, color)
}
