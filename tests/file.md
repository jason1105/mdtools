---
title: "30天自制操作系统"
tags: [学习笔记, 学习, 笔记]
draft: false
description: "30天自制操作系统"
source: "http://hrb.osask.jp/"
---
#操作系统 #30daysOS 

# 目的

制作512k启动区.

## 汇编

![[Pasted image 20211121105805.png]]

**常用的16位寄存器**

CPU 的这些小弟都是16位的, 加在一起不过16个字节.

```ASM
AX —— accumulator  # X 的意思是 extend
CX —— counter
DX —— data
BX —— base. The base of memory that program can use.  
SP —— stack pointer. Point to stack of program, used in conjunction with SS.
BP —— base pointer. Point to stack segment
SI —— source index. Point to memory location in the data segment addressed by DS
DI —— destination index. Same to SI, access memory location address by ES
```

BX/BP/SI/DI can be used for read or write memory, e.g. `MOV AL, BYTE [BX]`

**8个8位寄存器**

注意啊, 这8个寄存器实际上使用的还是 AX / CX / DX/ BX, CPU 并没有因为这8个寄存器而多了8个字节.

```ASM
AL —— accumulator low
CL —— counter low
DL —— data low
BL —— base low
AH —— accumulator high
CH —— counter high
DH —— data high
BH —— base high
```
