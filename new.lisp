(blink (+ 1 2) 500)
(define newfunc (lambda (a b) (+ a (- a b))))
(blink (newfunc 1 2) 500)
