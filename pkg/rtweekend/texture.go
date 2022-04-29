package rtweekend

import (
	"math"
)

type Texture interface {
	Value(u, v float64, p *Point3) *Color
}

type SolidColor struct {
	colorValue Color
}

type CheckerTexture struct {
	odd  interface{}
	even interface{}
}

type NoiseTexture struct {
	noise Perlin
}

/* Returns a pointer of SolidColor
   Params
       c Color     Color for texture
*/
func NewSolidColor(c Color) *SolidColor {
	return &SolidColor{c}
}

/* Returns a pointer of SolidColor
   Params
       r float64   Value to add red to color
       g float64   Value to add green to color
       b float64   Value to add blue to color
*/
func NewSolidColorRGB(r, g, b float64) *SolidColor {
	return NewSolidColor(*NewColor(r, b, b))
}

/* Returns a pointer of CheckerTexture
   Params
       even Texture   Texture for even blocks
       odd  Texture   Texture for odd blocks
*/
func NewCheckerTexture(even, odd Texture) *CheckerTexture {
	return &CheckerTexture{odd, even}
}

/* Returns a pointer of CheckerTexture
   Params
       even Color   Color of texture for even blocks
       odd  Color   Color of texture for odd blocks
*/
func NewCheckerTextureFromColor(c1, c2 Color) *CheckerTexture {
	return NewCheckerTexture(*NewSolidColor(c1), *NewSolidColor(c2))
}

// Returns a poiner of NoiseTexture
func NewNoiseTexture() *NoiseTexture {
	nt := new(NoiseTexture)
	nt.noise = *NewPerlin()

	return nt
}

func (sc SolidColor) Value(u, v float64, p *Point3) *Color {
	return &sc.colorValue
}

func (ct CheckerTexture) Value(u, v float64, p *Point3) *Color {
	sines := math.Sin(10*p.X()) * math.Sin(10*p.Y()) * math.Sin(10*p.Z())

	if sines < 0 {
		return ct.odd.(Texture).Value(u, v, p)
	} else {
		return ct.even.(Texture).Value(u, v, p)
	}
}

func (nt NoiseTexture) Value(u, v float64, p *Point3) *Color {
	return NewColor(1, 1, 1).Multiply(nt.noise.Noise(p))
}
