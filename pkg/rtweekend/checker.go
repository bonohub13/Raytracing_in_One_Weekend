package rtweekend

import (
	"math"
)

type CheckerTexture struct {
	odd  interface{}
	even interface{}
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

func (ct CheckerTexture) Value(u, v float64, p *Point3) *Color {
	sines := math.Sin(10*p.X()) * math.Sin(10*p.Y()) * math.Sin(10*p.Z())

	if sines < 0 {
		return ct.odd.(Texture).Value(u, v, p)
	} else {
		return ct.even.(Texture).Value(u, v, p)
	}
}
