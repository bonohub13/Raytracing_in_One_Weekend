package librt

type Camera struct {
	origin          Point3
	lowerLeftCorner Point3
	horizontal      Vec3
	vertical        Vec3
}

func NewCamera() *Camera {
	camera := &Camera{}
	aspectRatio := 16.0 / 9.0
	viewportHeight := 2.0
	viewportWidth := aspectRatio * viewportHeight
	focalLength := 1.0

	camera.origin = *NewPoint3(0, 0, 0)
	camera.horizontal = *NewVec3(viewportWidth, 0, 0)
	camera.vertical = *NewVec3(0, viewportHeight, 0)
	camera.lowerLeftCorner = *camera.origin.Substract(
		camera.horizontal.DivFloat64(2),
	).Substract(
		camera.vertical.DivFloat64(2),
	).Substract(
		NewVec3(0, 0, focalLength),
	)

	return camera
}

func (cam *Camera) GetRay(u, v float64) *Ray {
	return NewRay(
		cam.origin,
		*cam.lowerLeftCorner.Add(
			cam.horizontal.MultFloat64(u),
		).Add(
			cam.vertical.MultFloat64(v),
		).Substract(&cam.origin))
}
