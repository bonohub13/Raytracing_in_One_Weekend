package rtweekend

import (
	"errors"
	"fmt"
	image "image"
	_ "image/jpeg"
	_ "image/png"
	"math"
	"os"
)

type ImageTexture struct {
	image  image.Image
	mirror bool
	max    [2]float64 // Max values for x and y axis (x, y)
}

/* GPL-3.0
   NewImageTexture from https://github.com/DanielPetterson/solstrale/material/texture.go
   Params
       image   image.Image     Image for image texture
       mirror  bool            Mirror image
*/
func NewImageTexture(image image.Image, mirror bool) *ImageTexture {
	it := new(ImageTexture)
	max := image.Bounds().Max

	it.image = image
	it.mirror = mirror
	it.max = [2]float64{float64(max.X - 1), float64(max.Y - 1)}

	return it
}

/* GPL-3.0
   LoadNewImageTexture from https://github.com/DanielPetterson/solstrale/material/texture.go
   Params
       filepath string     Path of image file to import
*/
func LoadImageTexture(filepath string) (*ImageTexture, error) {
	f, err := os.Open(filepath)
	defer f.Close()

	if err != nil {
		return nil, errors.New(fmt.Sprintf(
			"Failed to load image texture %v. Got Error: %v",
			filepath, err.Error(),
		))
	}

	img, _, err := image.Decode(f)
	if err != nil {
		return nil, errors.New(fmt.Sprintf(
			"Failed to decode image texture %v. Got error: %v",
			filepath, err.Error(),
		))
	}

	return NewImageTexture(img, false), nil
}

func (*ImageTexture) BytesPerPixel() int {
	return 3
}

func (it ImageTexture) Value(u, v float64, p *Vec3) *Color {
	// Return cyan pixel by default if image is not imported
	if it.image == nil {
		return NewColor(0, 1, 1)
	}

	u = math.Mod(math.Abs(u), 1)
	v = math.Mod(math.Abs(v), 1)
	if it.mirror {
		u = 1 - u
	}

	x := int(u * it.max[0])
	y := int(u * it.max[1])

	r, g, b, _ := it.image.At(x, y).RGBA()

	r = r >> 8
	g = g >> 8
	b = b >> 8

	return NewColor(float64(r)/255, float64(g)/255, float64(b)/255)
}
