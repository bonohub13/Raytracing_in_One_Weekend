package rtweekend

type SolidColor struct {
	colorValue Color
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

func (sc SolidColor) Value(u, v float64, p *Point3) *Color {
	return sc.colorValue.MultiplyVertices(NewColor(1, 1, 1))
}
