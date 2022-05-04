package rtweekend

type Hittable interface {
	Hit(r *Ray, t_min, t_max float64, rec *HitRecord) bool
	BoundingBox(time0, time1 float64, outputBox *AABB) bool
}
