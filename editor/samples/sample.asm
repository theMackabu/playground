; Sample x86 Assembly File (Intel Syntax)
; This program calculates the factorial of a number

section .data
	 prompt db "Enter a number (1-8): ", 0
	 prompt_len equ $ - prompt
	 result_msg db "Factorial is: ", 0
	 result_msg_len equ $ - result_msg
	 newline db 10, 0

section .bss
	 input resb 2
	 factorial resq 1

section .text
	 global _start

_start:
	 ; Print prompt
	 mov eax, 4
	 mov ebx, 1
	 mov ecx, prompt
	 mov edx, prompt_len
	 int 0x80

	 ; Read input
	 mov eax, 3
	 mov ebx, 0
	 mov ecx, input
	 mov edx, 2
	 int 0x80

	 ; Convert ASCII to integer
	 movzx eax, byte [input]
	 sub eax, '0'

	 ; Calculate factorial
	 mov ebx, eax
	 mov ecx, 1
	 call calculate_factorial

	 ; Convert result to ASCII
	 mov eax, [factorial]
	 call int_to_ascii

	 ; Print result message
	 mov eax, 4
	 mov ebx, 1
	 mov ecx, result_msg
	 mov edx, result_msg_len
	 int 0x80

	 ; Print factorial result
	 mov eax, 4
	 mov ebx, 1
	 mov ecx, factorial
	 mov edx, 8
	 int 0x80

	 ; Print newline
	 mov eax, 4
	 mov ebx, 1
	 mov ecx, newline
	 mov edx, 1
	 int 0x80

	 ; Exit program
	 mov eax, 1
	 xor ebx, ebx
	 int 0x80

calculate_factorial:
	 cmp ebx, 1
	 jle .done
	 imul ecx, ebx
	 dec ebx
	 jmp calculate_factorial
.done:
	 mov [factorial], ecx
	 ret

int_to_ascii:
	 mov ecx, 7
	 mov ebx, 10
.loop:
	 xor edx, edx
	 div ebx
	 add dl, '0'
	 mov [factorial + ecx], dl
	 dec ecx
	 test eax, eax
	 jnz .loop
.pad:
	 mov byte [factorial + ecx], ' '
	 loop .pad
	 ret