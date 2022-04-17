package rtweekendlib

import (
	"container/list"
)

type HittableList struct {
	objects list.List
}

func (hl *HittableList) Clear() {
	hl.objects = *new(list.List)
}

func (hl *HittableList) Add(object Hittable) {
	hl.objects.PushBack(object)
}

func (hl *HittableList) Hit(r *Ray, t_min, t_max float64, rec *HitRecord) bool {
	tempRec := new(HitRecord)
	hitAnything := false
	closestSoFar := t_max

	for object := hl.objects.Front(); object != nil; object = object.Next() {
		if object.Value.(Hittable).Hit(r, t_min, closestSoFar, tempRec) {
			hitAnything = true
			closestSoFar = tempRec.T()
			rec = tempRec
		}
	}

	return hitAnything
}

func NewHittableList(object Hittable) *HittableList {
	hl := new(HittableList)

	hl.Add(object)

	return hl
}
