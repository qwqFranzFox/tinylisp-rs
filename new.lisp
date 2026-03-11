(define func (lambda (a b) (+ (+ a b) (- a b))))
(define gcd (lambda (a b) (if (eq? b 0) a (gcd b (mod a b)))))
(func 10 10)
(gcd 24 27)
