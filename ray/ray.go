package ray

import (
	"github.com/bonohub13/Raytracing_in_One_Weekend/vec3"
)

type Ray struct {
	orig vec3.Point3
	dir  vec3.Vec3
}

func NewRay(origin *vec3.Point3, direction *vec3.Vec3) *Ray {
	ray := new(Ray)

	ray.orig = *origin
	ray.dir = *direction

	return ray
}

func (r *Ray) Origin() vec3.Point3  { return r.orig }
func (r *Ray) Direction() vec3.Vec3 { return r.dir }

func (r *Ray) At(gain float64) *vec3.Point3 {
	return r.orig.Add(r.dir.Multiply(gain))
}
