package rtweekend

type HittableList struct {
	objects []Hittable
}

func NewHittableList(object Hittable) *HittableList {
	hl := new(HittableList)

	hl.objects = make([]Hittable, 0)
	hl.Add(object)

	return hl
}

func (hl *HittableList) Clear() {
	hl.objects = []Hittable{}
}

func (hl *HittableList) Add(object Hittable) {
	hl.objects = append(hl.objects, object)
}

func (hl HittableList) Hit(r *Ray, t_min, t_max float64, rec *HitRecord) bool {
	tempRec := new(HitRecord)
	hitAnything := false
	closestSoFar := t_max

	for i := 0; i < len(hl.objects); i++ {
		if hl.objects[i].Hit(r, t_min, closestSoFar, tempRec) {
			hitAnything = true
			closestSoFar = tempRec.t
			*rec = *tempRec
		}
	}

	return hitAnything
}
