package rtweekend

import (
	"math"
)

// Camera object
type Camera struct {
	origin          Point3
	lowerLeftCorner Point3
	horizontal      Vec3
	vertical        Vec3
	u, v, w         Vec3
	lensRadius      float64
	time0, time1    float64
}

/* Returns pointer to a Camera object
   Params
       lookFrom     Point3      Coordinate where the camera is
       lookAt       Point3      Coordinate where the scene is
       vup          Vec3        Direction for view up
       vfov         float64     Vertical field-of-view in degrees
       aspectRatio  float64     Aspect ratio
       aperture     float64     Aperture
       focusDist    float64     Focus distance for camera
       time         []float64   Start and end point of time (both defaults to 0)
*/
func NewCamera(
	lookFrom, lookAt Point3,
	vup Vec3,
	vfov, aspectRatio, aperture, focusDist float64,
	time ...float64,
) *Camera {
	camera := new(Camera)

	if len(time) == 2 {
		camera.time0 = time[0]
		camera.time1 = time[1]
	} else if len(time) == 1 {
		camera.time0 = time[0]
		camera.time1 = 0
	} else {
		camera.time0 = 0
		camera.time1 = 0
	}
	theta := DegreesToRadians(vfov)
	h := math.Tan(theta / 2)
	viewportHeight := 2.0 * h
	viewportWidth := aspectRatio * viewportHeight

	camera.w = *UnitVector(lookFrom.Substract(&lookAt))
	camera.u = *UnitVector(Cross(&vup, &camera.w))
	camera.v = *Cross(&camera.w, &camera.u)

	camera.origin = lookFrom
	camera.horizontal = *camera.u.Multiply(focusDist * viewportWidth)
	camera.vertical = *camera.v.Multiply(focusDist * viewportHeight)
	camera.lowerLeftCorner = *camera.origin.Substract(
		camera.horizontal.Divide(2),
	).Substract(
		camera.vertical.Divide(2),
	).Substract(camera.w.Multiply(focusDist))
	camera.lensRadius = aperture / 2

	return camera
}

/* Computes the rays from camera viewpoint and angle
   Param
       s float64   Horizontal ray length
       t float64   Vertical ray length
*/
func (c *Camera) GetRay(s, t float64) *Ray {
	rd := RandomInUnitDisk().Multiply(c.lensRadius)
	offset := c.u.Multiply(rd.X()).Add(c.v.Multiply(rd.Y()))

	return NewRay(
		*c.origin.Add(offset),
		*c.lowerLeftCorner.Add(
			c.horizontal.Multiply(s),
		).Add(
			c.vertical.Multiply(t),
		).Substract(
			&c.origin,
		).Substract(offset),
		RandomFloat64InRange(c.time0, c.time1),
	)
}
