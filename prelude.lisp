(define newfunc (lambda (a b) (+ a (- a b))))
(define gcd (lambda (a b) (if (eq? b 0) a (gcd b (mod a b)))))
