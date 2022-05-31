// Axis-Aligned rectangle on XY axis
package rtweekend

type RectXY struct {
	material interface{}
	x0, x1   float64
	y0, y1   float64
	k        float64
}

func NewRectXY(x0, x1, y0, y1, k float64, material Material) *RectXY {
	rect := new(RectXY)

	rect.x0 = x0
	rect.x1 = x1
	rect.y0 = y0
	rect.y1 = y1
	rect.k = k
	rect.material = material

	return rect
}

func (rect RectXY) Hit(r *Ray, t_min, t_max float64, rec *HitRecord) bool {
	t := (rect.k - r.orig.Z()) / r.dir.Z()
	setHitRecord := func(x, y float64) {
		*rec = HitRecord{}
		rec.u = (x - rect.x0) / (rect.x1 - rect.x0)
		rec.v = (y - rect.y0) / (rect.y1 - rect.y0)
		rec.t = t
		outward_normal := NewVec3(0, 0, 1)
		rec.SetFaceNormal(r, outward_normal)
		rec.material = rect.material
		rec.p = *r.At(t)
	}

	if t_min <= t && t <= t_max {
		x := r.orig.X() + t*r.dir.X()
		y := r.orig.Y() + t*r.dir.Y()

		if rect.x0 <= x && x <= rect.x1 && rect.y0 <= y && y <= rect.y1 {
			setHitRecord(x, y)
			return true
		}
	}

	return false
}

func (rect RectXY) BoundingBox(time0, time1 float64, outputBox *AABB) bool {
	*outputBox = *NewAABB(
		*NewPoint3(rect.x0, rect.y0, rect.k-0.0001),
		*NewPoint3(rect.x1, rect.y1, rect.k-0.0001),
	)

	return true
}
