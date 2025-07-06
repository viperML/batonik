package modules

type Simple struct {
	Text string
}

func (s *Simple) Run() string {
	return s.Text
}

func S(text string) *Simple {
	return &Simple{Text: text}
}

func NewLine() *Simple {
	return &Simple{Text: "\n"}
}
