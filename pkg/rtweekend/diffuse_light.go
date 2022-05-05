package rtweekend

type DiffuseLight struct {
    emit interface{}
}

func NewDiffuseLight(emit Texture) *DiffuseLight {
    dl := new(DiffuseLight)

    dl.emit = emit

    return dl
}

func NewDiffuseLightFromColor(c Color) *DiffuseLight {
    sc := NewSolidColor(c)

    return &DiffuseLight{*sc}
}

func (dl DiffuseLight) Emitted(u, v float64, p *Point3) *Color {
    return dl.emit.(Texture).Value(u, v, p)
}

func (dl DiffuseLight) Scatter(
    r_in *Ray,
    rec *HitRecord,
    attenuation *Color,
    scattered *Ray,
) bool {
    return false
}
