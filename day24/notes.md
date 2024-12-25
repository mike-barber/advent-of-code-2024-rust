Unexpected result for bit 16 - input 1,1 got 1 carry 1; totals got 262144 expected 131072                             
Unexpected result for bit 17 - input 0,1 got 0 carry 0; totals got 262144 expected 131072                             
nexpected result for bit 17 - input 1,0 got 1 carry 1; totals got 262144 expected 131072      
Unexpected result for bit 17 - input 1,1 got 1 carry 1; totals got 131072 expected 262144

Problem first appears on bit 17.

z16 depends on added ["kbw", "x16", "wnf", "y16", "ttr" kbj"]
z17 depends on added ["qwg", "x17", "y17", "qwd", "tfc", "wvj", "qhq", "swm"]
z18 depends on added ["fwm", "cmv", "x18", "y18"]

kbw XOR wnf -> z16
y16 XOR x16 -> wnf (s1)
x16 AND y16 -> qwd (c1)

kbj OR ttr -> kbw (c2 prev?)

s1 = x ^ y      x16 ^ y16 -> wnf
s2 = s1 ^ c0    wnf ^ kbw -> z16
c1 = x & y      x16 & y16 -> qwd
ci = s1 & c0    wnf & kbw -> swm
c2 = ci + c1    swm | qwd -> qwg

------

s1 = x ^ y      x17 ^ y17 -> wvj
s2 = s1 ^ c0    wvj ^ qwg -> cmv (should be z17)
c1 = x & y      x17 & y17 -> tfc
ci = s1 & c0    wvj & qwg -> qhq
c2 = ci + c1    qhq | tfc -> z17 (should be ... cmv?)

Swap z17 and cmv worked and fixes bit 17.



Unexpected result for bit 22 - input 1,1 got 1 carry 1; totals got 16777216 expected 8388608
Unexpected result for bit 23 - input 0,1 got 0 carry 0; totals got 16777216 expected 8388608
Unexpected result for bit 23 - input 1,0 got 1 carry 1; totals got 16777216 expected 8388608

bit 22... 

s1 = x ^ y      
s2 = s1 ^ c0    
c1 = x & y      tgj = x22 & y22
ci = s1 & c0    
c2 = ci + c1    kkf = fcp | tgj --> carries to 23 probably

bit 23...

s1 = x ^ y      pbw = x23 XOR y23
s2 = s1 ^ c0    rmj = pbw XOR kkf  (rmj != z23; conflicts; tgj is carry from prev?)
c1 = x & y      frg = x23 AND y23
ci = s1 & c0    z23 = kkf AND pbw  (z23 is not carry!)
c2 = ci + c1    pkh = rmj OR frg 

swap rmj and z23

-----------------------

bit 29

    x28 AND y28 -> rjr
    ctp OR rjr -> msw
    c0 is msw from previous?

s1 = x ^ y      x29 XOR y29 -> qsg
s2 = s1 ^ c0    qsg XOR msw -> z29 
c1 = x & y      x29 AND y29 -> dhk
ci = s1 & c0    qsg AND msw -> ntq
c2 = ci + c1    ntq OR dhk -> knj (carry for 30)

bit 30

s1 = x ^ y      y30 XOR x30 -> rvp
s2 = s1 ^ c0    rvp XOR knj -> rdg (nope - should be z30)
c1 = x & y      x30 AND y30 -> z30 (nope - should be c1.)
ci = s1 & c0    
c2 = ci + c1    

---------------

bit 38

s1 = x ^ y      x38 XOR y38 -> btb
s2 = s1 ^ c0    tsk XOR mwp -> z38 (not using s1=btb?)
c1 = x & y      x38 AND y38 -> mwp (not "mwp" since should be in OR)
ci = s1 & c0    mwp AND tsk -> jhw
c2 = ci + c1    jhw OR btb -> dqg

swap btb / mwp? - ~~yep~~