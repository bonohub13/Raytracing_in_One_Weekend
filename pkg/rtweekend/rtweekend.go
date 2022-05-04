package rtweekend

import (
	"math"
	"math/rand"
	"time"
)

const (
	INFINITY = math.MaxFloat64
	PI       = 3.1415926535897932385
)

func DegreesToRadians(degrees float64) float64 {
	return degrees * PI / 180
}

func InitRandom() {
	rand.Seed(time.Now().UnixNano())
}

func RandomFloat64() float64 {
	return rand.Float64()
}

func RandomFloat64InRange(min, max float64) float64 {
	return min + (max-min)*RandomFloat64()
}

func RandomIntInRange(min, max int) int {
	return int(RandomFloat64InRange(float64(min), float64(max)+1))
}

func Clamp(x, min, max float64) float64 {
	if x < min {
		return min
	} else if x > max {
		return max
	} else {
		return x
	}
}
