# Leo optimization

It's best to show this one through a number of examples.

Example grammar:

start ::= rr
rr ::= rep rr | rep

Example input:

rep rep rep rep

Items:

recognizer.item: start ::= 0 () rr
recognizer.item: rr ::= 0 () rep rr
recognizer.item: rr ::= 0 () rep

recognizer.scan: 0 rep 1
recognizer.item: rr ::= 0 rep (1) rr
recognizer.complete: rr ::= 0 rep (1)
recognizer.complete: start ::= 0 rr (1)
recognizer.predict: rr ::= (1) rep
recognizer.predict: rr ::= (1) rep rr
leo_item: rr ::= 0 rep (1) rr

recognizer.scan: 1 rep 2
recognizer.complete: rr ::= 1 rep (2)
recognizer.item: rr ::= 1 rep (2) rr
recognizer.complete LEO: rr ::= 0 rep 1 rr (2)
    PREVENTED: recognizer.complete start ::= 0 rr (2)
    LeoProduct rr: { left: rep 0..1, right: rr 1..2 }
recognizer.predict: rr ::= (2) rep
recognizer.predict: rr ::= (2) rep rr

recognizer.scan: 2 rep 3
recognizer.complete: rr ::= rep (3)
recognizer.item: rr ::= 2 rep (3) rr
recognizer.complete leo: rr ::= rep rr ()
LeoProduct { left: rr 0..2,  right: rr 2..3 }
