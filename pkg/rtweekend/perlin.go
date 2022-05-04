package rtweekend

import (
	"math"
)

type Perlin struct {
	ranvec              []Vec3
	permX, permY, permZ []int
}

func (*Perlin) POINT_COUNT() int {
	return 256
}

/* Return pointer of Perlin object
 */
func NewPerlin() *Perlin {
	perlin := new(Perlin)

	perlin.ranvec = make([]Vec3, perlin.POINT_COUNT())
	perlin.permX = perlin.perlinGeneratePerm()
	perlin.permY = perlin.perlinGeneratePerm()
	perlin.permZ = perlin.perlinGeneratePerm()

	for i := 0; i < perlin.POINT_COUNT(); i++ {
		perlin.ranvec[i] = *UnitVector(RandomVec3InRange(-1, 1))
	}

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
	var c [2][2][2]Vec3

	for di := 0; di < 2; di++ {
		for dj := 0; dj < 2; dj++ {
			for dk := 0; dk < 2; dk++ {
				c[di][dj][dk] = p.ranvec[p.permX[(i+di)&255]^
					p.permY[(j+dj)&255]^
					p.permZ[(k+dk)&255]]
			}
		}
	}

	return p.trilinearInterp(&c, u, v, w)
}

func (perlin *Perlin) Turb(p Point3, depth ...int) float64 {
	var depth_ int
	accum := 0.0
	weight := 1.0

	if len(depth) == 1 {
		depth_ = depth[0]
	} else {
		depth_ = 7
	}

	for i := 0; i < depth_; i++ {
		accum += weight * perlin.Noise(&p)
		weight *= 0.5
		p.MultToThis(2)
	}

	return math.Abs(accum)
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
func (p *Perlin) trilinearInterp(c *[2][2][2]Vec3, u, v, w float64) float64 {
	accum := 0.0
	uu := u * u * (3 - 2*u)
	vv := v * v * (3 - 2*v)
	ww := w * w * (3 - 2*w)

	for i := 0; i < 2; i++ {
		for j := 0; j < 2; j++ {
			for k := 0; k < 2; k++ {
				weightV := NewVec3(u-float64(i), v-float64(j), w-float64(k))
				accum += (float64(i)*uu + float64(1-i)*(1-uu)) *
					(float64(j)*vv + float64(1-j)*(1-vv)) *
					(float64(k)*ww + float64(1-k)*(1-ww)) *
					Dot(&(*c)[i][j][k], weightV)
			}
		}
	}

	return accum
}
