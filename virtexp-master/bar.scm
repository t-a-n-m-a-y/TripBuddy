(define (memory-limit-exceeded mem)
    (begin
        (displayln "In memory limit event handler")
        'suspend))
(define (compute-time-exceeded time)
    (begin
        'suspend))
(define (get-memory info)
    (car info))
(define (generate info)
    (begin
        (displayln info)
        (if (> (get-memory info) 1730560)
            'memory-limit-exceeded
            'noop)))	
