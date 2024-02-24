AS = as
OBJS = hello.o
BINS = hello.bin
TARGETS = exec

.PHONY: all
all: $(BINS) $(TARGETS)

%.o: %.s
	$(AS) -o $@ $<

%.bin: %.o
	objcopy --output-target=binary --only-section=.text $< $@

exec: exec.c
	gcc -g -o exec exec.c

.PHONY: clean
clean:
	rm -f $(OBJS) $(BINS)