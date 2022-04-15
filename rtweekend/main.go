package main

import (
	"fmt"
)

// 定数
const (
	IMAGE_WIDTH  = 256
	IMAGE_HEIGHT = 256
)

func main() {
	fmt.Printf("P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)

	for j := IMAGE_HEIGHT - 1; j >= 0; j-- {
		for i := 0; i < IMAGE_WIDTH; i++ {
			r := float64(i) / float64(IMAGE_WIDTH-1)
			g := float64(j) / float64(IMAGE_HEIGHT-1)
			b := 0.25

			ir := int(255.999 * r)
			ig := int(255.999 * g)
			ib := int(255.999 * b)

			fmt.Printf("%d %d %d\n", ir, ig, ib)
		}
	}
}
