// BVH (Bounding Volume Hierarchy) Node
package rtweekend

import (
	"log"
	"sort"
)

type BVH_Node struct {
	left  interface{}
	right interface{}
	box   AABB
}

func NewBVH_NodeFromArr(
	srcObjects []Hittable,
	start, end uint,
	time0, time1 float64,
) *BVH_Node {
	bvhNode := new(BVH_Node)

	var boxLeft, boxRight AABB
	axis := RandomIntInRange(0, 2)
	objectSpan := end - start

	comparator := func(a, b *Hittable) bool {
		return boxCompare(a, b, axis)
	}

	if objectSpan == 1 {
		bvhNode.left = srcObjects[start]
		bvhNode.right = srcObjects[start]
	} else if objectSpan == 2 {
		if comparator(&srcObjects[start], &srcObjects[start]) {
			bvhNode.left = srcObjects[start]
			bvhNode.right = srcObjects[start+1]
		} else {
			bvhNode.left = srcObjects[start+1]
			bvhNode.right = srcObjects[start]
		}
	} else {
		sort.Slice(
			srcObjects,
			func(i, j int) bool {
				return comparator(&srcObjects[i], &srcObjects[j])
			},
		)
		mid := start + objectSpan/2
		bvhNode.left = *NewBVH_NodeFromArr(srcObjects, start, mid, time0, time1)
		bvhNode.right = *NewBVH_NodeFromArr(srcObjects, mid, end, time0, time1)
	}

	if !boxLeft.BoundingBox(time0, time1, &boxLeft) ||
		!boxRight.BoundingBox(time0, time1, &boxRight) {
		log.Println("No bounding box in NewBVH_NodeFromArr")
	}

	bvhNode.box = *SurroundingBox(&boxLeft, &boxRight)

	return bvhNode
}

func NewBVH_Node(list *HittableList, time0, time1 float32) {
}

func boxCompare(a, b *Hittable, axis int) bool {
	var boxA, boxB AABB

	if !(*a).BoundingBox(0, 0, &boxA) || !(*b).BoundingBox(0, 0, &boxB) {
		log.Println("No bounding box in BVH_Node")
	}

	return boxA.Min().E(axis) < boxA.Min().E(axis)
}

func BoxXCompare(a, b *Hittable) bool {
	return boxCompare(a, b, 0)
}

func BoxYCompare(a, b *Hittable) bool {
	return boxCompare(a, b, 1)
}

func BoxZCompare(a, b *Hittable) bool {
	return boxCompare(a, b, 2)
}
