(define (memory-limit-exceeded mem)
    (begin
    	(displayln "Memory limit exceeded")
        (displayln "In memory limit event handler")
        '(migrate qemu+ssh://rahul@192.168.95.79/system)))
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
