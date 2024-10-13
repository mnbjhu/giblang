package main
import "fmt"
type T7 struct {
}

type T9 interface {
T8() string
}
func (self T12) T8() string {return "Thing String"
}
type T12 struct {
test int
even_more string
}
type T13 struct {
inner T12
}
func T14(s T9)  {fmt.Print(s.T8())
fmt.Print("\n")
}
func main()  {var test T9
test = T12{test: 123,even_more: "Hello",}
T14(test)
}
type T19 struct {M16 *T16
M17 *T17
M18 *T18
}
type T16 struct {
}
type T17 struct {
F0 int
}
type T18 struct {
some int
other string
}

