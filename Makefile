RUSTC=rustc
RCFLAGS=-O
SRCDIR=src/Mi
SRC=main.rs
INSTALL   ?= install
MKDIR     ?= $(INSTALL) -d
BINDIR    ?= $(PREFIX)/bin
DESTDIR   ?=

r:	$(SRCDIR)
	cd $^ && $(RUSTC) -o ../../Mi $(SRC) ${RCFLAGS}

.PHONY: clean rebuild

rebuild: clean | r

clean:
	@echo " --- Clean binaries --- "
	rm -f Mi
	@echo " --- Clean temp files --- "
	find . -name '*~' -delete;
	find . -name '#*#' -delete;

install:
	$(MKDIR) $(DESTDIR)$(BINDIR)
	$(INSTALL) Mi$(EXE) $(DESTDIR)$(BINDIR)/
