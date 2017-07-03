all: dissasm

dissasm: dissasm.o
	gcc dissasm.o -o dissasm

dissasm.o:
	gcc -c dissasm.c

clean:
	rm -rf *.o dissasm

