package common

import (
	"github.com/bonohub13/Raytracing_in_One_Weekend/ray"
	"github.com/bonohub13/Raytracing_in_One_Weekend/vec3"
)

func RayColor(r *ray.Ray) *vec3.Color {
	dir := r.Direction()
	unitDirection := vec3.UnitVector(&dir)
	t := 0.5 * (unitDirection.Y() + 1.0)
	c0 := vec3.NewColor(1.0, 1.0, 1.0)
	c1 := vec3.NewColor(0.5, 0.7, 1.0)
	c0.MultToThis(1.0 - t)
	c1.MultToThis(t)

	return c0.Add(c1)
}
