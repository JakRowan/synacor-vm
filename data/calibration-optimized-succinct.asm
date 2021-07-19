0x0000:    NOOP
0x0000:    NOOP
0x0002:     SET    FX      0x0000
0x0003:      JF    HX      0x000C
0x0008:     SET    AX      0x0002
0x000B:     RET

0x000C:     POP    AX
0x000E:     POP    BX
0x0010:     ADD    FX      FX      0x7FFE
0x0014:      EQ    EX      BX      0x0001
0x0018:      JF    EX      0x0033
0x001B:      JF    FX      0x0045
0x001E:     POP    CX
0x0020:     POP    DX
0x000C:     ADD    FX      0x7FFE
0x0025:      EQ    EX      DX      0x0002
0x0029:      JT    EX      0x004E
0x002C:     ADD    EX      CX      0x7FFF
0x0030:      JT    EX      0x0067

0x0033:    PUSH    BX
0x0035:    PUSH    BX
0x0037:     ADD    FX      FX      0x0002
0x003B:     ADD    EX      BX      0x7FFF
0x003F:    PUSH    EX
0x0041:    PUSH    HX
0x0043:     JMP    0x000C

0x0045:     ADD    EX      AX      0x0001
0x0049:     ADD    AX      HX      EX
0x004D:     RET

0x004E:     ADD    EX      DX      0x7FFF
0x0052:    PUSH    EX
0x0054:     ADD    EX      HX      0x0001
0x0058:    MULT    EX      EX      CX
0x005C:     ADD    EX      EX      AX
0x0060:    PUSH    EX
0x0062:     ADD    FX      0x7FFE
0x0066:     JMP    0x000C

0x0067:    PUSH    DX
0x0069:     ADD    EX      CX      0x7FFF
0x006D:    PUSH    EX
0x006F:     ADD    FX      FX      0x0002

0x0073:     ADD    EX      DX      0x7FFF
0x0077:    PUSH    EX
0x0079:     ADD    EX      HX      0x0001
0x007D:     ADD    EX      EX      AX
0x0081:    PUSH    EX
0x0083:     ADD    FX      FX      0x0002
0x0087:     JMP    0x000C

0x0089:    HALT
