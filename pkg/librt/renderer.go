package librt

import (
	"fmt"
	"image"
	"image/png"
	"os"
	"regexp"
	"sync"
)

type renderer struct {
	imageWidth     int
	imageHeight    int
	samplePerPixel int
	depth          int
	filename       string
	world          HittableList
	camera         Camera
}

type renderedLine struct {
	row    int
	pixels string
}

func NewRenderer(
	imageWidth, imageHeight, samplePerPixel, depth int,
	filename string,
	camera *Camera,
	world *HittableList,
) *renderer {
	var r renderer

	r.imageWidth = imageWidth
	r.imageHeight = imageHeight
	r.samplePerPixel = samplePerPixel
	r.depth = depth
	if len(regexp.MustCompile(`\.png$`).FindAllString(filename, -1)) > 0 {
		r.filename = filename
	} else {
		r.filename = filename + ".png"
	}
	r.camera = *camera
	r.world = *world

	return &r
}

/* Source code from https://github.com/i-am-g2/Tr/blob/master/main.go
   Author: i-am-g2
   License: Non License
*/
func (r *renderer) Render() {
	img := image.NewRGBA(
		image.Rectangle{
			image.Point{0, 0},
			image.Point{r.imageWidth, r.imageHeight},
		},
	)
	var wg sync.WaitGroup

	for j := 0; j < r.imageHeight; j++ {
		j := j

		r.progressBar(j, r.imageHeight)
		wg.Add(1)
		go func() {
			for i := 0; i < r.imageWidth; i++ {
				i, j := i, j

				wg.Add(1)
				go func() {
					pixelColor := NewColor(0, 0, 0)
					for s := 0; s < r.samplePerPixel; s++ {
						u := (float64(i) + RandomFloat64()) / float64(r.imageWidth-1)
						v := (float64(j) + RandomFloat64()) / float64(r.imageHeight-1)
						ray := r.camera.GetRay(u, v)
						pixelColor = pixelColor.Add(RayColor(ray, &r.world, r.depth))
					}

					WriteColor(i, j, r.samplePerPixel, pixelColor, img)
					wg.Done()
				}()
			}
		}()
		wg.Done()
		wg.Wait()
	}

	pngFile, _ := os.Create(r.filename)
	png.Encode(pngFile, img)
	pngFile.Close()
}

/* Source code from https://github.com/i-am-g2/Tr/blob/master/tr/utils.go
   Author: i-am-g2
   License: Non License
*/
func (r *renderer) progressBar(done, total int) {
	scale := 50.0
	percentDone := int((float64(done) / float64(total)) * scale)

	fmt.Fprintf(os.Stderr, "Rendering [")
	for i := 0; i < percentDone; i++ {
		fmt.Fprintf(os.Stderr, "=")
	}
	for i := percentDone; i < int(scale); i++ {
		fmt.Fprintf(os.Stderr, " ")
	}
	fmt.Fprintf(os.Stderr, "] %d out of %d \r", done+1, total)
	if done+1 == total {
		fmt.Fprintf(os.Stderr, "\n")
	}
}
