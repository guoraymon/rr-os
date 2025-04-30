# SUBDIRS := hello_world bad_address bad_instructions
SUBDIRS := yield_a yield_b yield_c
BASE := 0x80400000
OFFSET := 0x20000

bin:
	@i=0; \
	for dir in $(SUBDIRS); do \
		cp $$dir/linker.ld $$dir/linker.ld.bak; \
		addr=$$(( $(BASE) + $(OFFSET) * $$i )); \
		printf "Updating $$dir/linker.ld to address 0x%x\n" $$addr; \
		sed -i "s/$(BASE)/0x$$(printf '%x' $$addr)/g" $$dir/linker.ld; \
		$(MAKE) -C $$dir bin || exit $$?; \
		mv $$dir/linker.ld.bak $$dir/linker.ld; \
		i=$$(( $$i + 1 )); \
	done

clean:
	@for dir in $(SUBDIRS); do \
		$(MAKE) -C $$dir clean; \
	done
	
.PHONY: bin
