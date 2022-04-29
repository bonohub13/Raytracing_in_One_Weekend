package rtweekend

import (
	"math"
)

type Perlin struct {
	ranfloat            []float64
	permX, permY, permZ []int
}

func (*Perlin) POINT_COUNT() int {
	return 256
}

/* Return pointer of Perlin object
 */
func NewPerlin() *Perlin {
	perlin := new(Perlin)

	perlin.ranfloat = make([]float64, perlin.POINT_COUNT())

	for i := 0; i < perlin.POINT_COUNT(); i++ {
		perlin.ranfloat[i] = RandomFloat64()
	}

	perlin.permX = perlin.perlinGeneratePerm()
	perlin.permY = perlin.perlinGeneratePerm()
	perlin.permZ = perlin.perlinGeneratePerm()

	return perlin
}

/* Output random noise
   Param
       p_ *Point3  Coordinate to add noise
*/
func (p *Perlin) Noise(p_ *Point3) float64 {
	u := p_.X() - math.Floor(p_.X())
	v := p_.Y() - math.Floor(p_.Y())
	w := p_.Z() - math.Floor(p_.Z())
	i := int(math.Floor(p_.X()))
	j := int(math.Floor(p_.Y()))
	k := int(math.Floor(p_.Z()))
	var c [2][2][2]float64

	u = u * u * (3 - 2*u)
	v = v * v * (3 - 2*v)
	w = w * w * (3 - 2*w)

	for di := 0; di < 2; di++ {
		for dj := 0; dj < 2; dj++ {
			for dk := 0; dk < 2; dk++ {
				c[di][dj][dk] = p.ranfloat[p.permX[(i+di)&255]^
					p.permY[(j+dj)&255]^
					p.permZ[(k+dk)&255]]
			}
		}
	}

	return p.trilinearInterp(&c, u, v, w)
}

// Generate Perlin noise
func (p *Perlin) perlinGeneratePerm() []int {
	retval := make([]int, p.POINT_COUNT())

	for i := 0; i < p.POINT_COUNT(); i++ {
		retval[i] = i
	}

	p.permute(&retval, p.POINT_COUNT())

	return retval
}

/* Permute data in Perlin
   p_  *[]int  Data to permute
   n   int     Size of data
*/
func (p *Perlin) permute(p_ *[]int, n int) {
	for i := n - 1; i > 0; i-- {
		target := RandomIntInRange(0, i)
		tmp := (*p_)[i]
		(*p_)[i] = (*p_)[target]
		(*p_)[target] = tmp
	}
}

/* Trilinear inpolation
   Params
       c *[2][2][2]float64 3D array to calculate interpolation
       u float64           Gain for x axis
       v float64           Gain for y axis
       w float64           Gain for z axis
*/
func (p *Perlin) trilinearInterp(c *[2][2][2]float64, u, v, w float64) float64 {
	accum := 0.0

	for i := 0; i < 2; i++ {
		for j := 0; j < 2; j++ {
			for k := 0; k < 2; k++ {
				accum += (float64(i)*u + float64(1-i)*(1-u)) *
					(float64(j)*v + float64(1-j)*(1-v)) *
					(float64(k)*w + float64(1-k)*(1-w)) * (*c)[i][j][k]
			}
		}
	}

	return accum
}
