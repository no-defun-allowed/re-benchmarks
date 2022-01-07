(ql:quickload '(:one-more-re-nightmare-simd :the-cost-of-nothing))

(defvar *log* (coerce (alexandria:read-file-into-string "Xorg.0.log") 'simple-base-string))

(defmacro bench (name size form)
  `(let ((time-taken (the-cost-of-nothing:benchmark ,form :timeout 5)))
     (format *trace-output* "~&~a~&  " ,name)
     (the-cost-of-nothing:write-si-unit time-taken "seconds" *trace-output*)
     (format *trace-output* "~&  ")
     (the-cost-of-nothing:write-si-unit (/ ,size time-taken) "characters/second" *trace-output*)
     time-taken))

(bench "log, no submatches"
       (length *log*)
       (one-more-re-nightmare:all-matches "[0-9]+x[0-9]+" *log*))
(bench "log, submatches"
       (length *log*)
       (one-more-re-nightmare:all-matches "«[0-9]+»x«[0-9]+»" *log*))
(bench "quoted log, no submatches"
       (length *log*)
       (one-more-re-nightmare:all-matches "\"[0-9]+x[0-9]+\"" *log*))
(bench "quoted log, submatches"
       (length *log*)
       (one-more-re-nightmare:all-matches "\"«[0-9]+»x«[0-9]+»\"" *log*))
