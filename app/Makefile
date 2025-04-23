SUBDIRS := $(shell find . -mindepth 1 -maxdepth 1 -type d)

bin: $(SUBDIRS)

$(SUBDIRS):
	@$(MAKE) -C $@ bin

.PHONY: all $(SUBDIRS)