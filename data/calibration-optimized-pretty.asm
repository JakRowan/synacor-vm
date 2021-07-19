# AX, BX, CX, DX, HX = Working Registers
# EX = General Register
# FX = Stack Counter
# GX = Return Value

#init
0x00000:     SET   HX      0x6486
0x00003:    PUSH   0x1
0x00005:    PUSH   HX
0x00007:    PUSH   0x2
0x00009:    PUSH   HX
0x0000B:    PUSH   0x3
0x0000D:    PUSH   HX
0x0000F:    PUSH   0x4
0x00011:    PUSH   0x1

0x00013:    CALL   0x0017
0x00015:      IN
0x00016:    HALT

#start
0x00017:    NOOP
0x00018:     SET   FX      0x0
0x0001B:      JF   HX      0x0022
0x0001E:     SET   AX      2
0x00021:     RET
#loop
0x00022:     POP   AX
0x00024:     POP   BX
0x00026:     ADD   FX      FX      0x7FFE
0x0002A:      EQ   EX      BX      0x1
0x0002E:      JF   EX      0x0049
0x00031:      JF   FX      0x005B
0x00034:     POP   CX
0x00036:     POP   DX
0x00038:     ADD   FX      0x7FFE
0x0003B:      EQ   EX      DX      0x2
0x0003F:      JT   EX      0x0064
0x00042:     ADD   EX      CX      0x7FFF
0x00046:      JT   EX      0x007D
#if_one
0x00049:    PUSH   BX
0x0004B:    PUSH   BX
0x0004D:     ADD   FX      FX      0x2
0x00051:     ADD   EX      BX      0x7FFF
0x00055:    PUSH   EX
0x00057:    PUSH   HX
0x00059:     JMP   0x0022
#cal_a
0x0005B:     ADD   EX      AX      0x1
0x0005F:     ADD   AX      HX      EX
0x00063:     RET
#cal_b
0x00064:     ADD   EX      DX      0x7FFF
0x00068:    PUSH   EX
0x0006A:     ADD   EX      HX      0x1
0x0006E:    MULT   EX      EX      CX
0x00072:     ADD   EX      EX      AX
0x00076:    PUSH   EX
0x00078:     ADD   FX      0x7FFE
0x0007B:     JMP   0x0022
#cal_c
0x0007D:    PUSH   DX
0x0007F:     ADD   EX      CX      0x7FFF
0x00083:    PUSH   EX
0x00085:     ADD   FX      FX      0x2
#cal_d
0x00089:     ADD   EX      DX      0x7FFF
0x0008D:    PUSH   EX
0x0008F:     ADD   EX      HX      0x1
0x00093:     ADD   EX      EX      AX
0x00097:    PUSH   EX
0x00099:     ADD   FX      FX      0x2
0x0009D:     JMP   0x0022

0x0009F:    HALT
