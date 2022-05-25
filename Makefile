.PHONY: all build strip install clean
BINARY=indexnow-update

all: build strip install

build:
	env PATH=${PATH}:${HOME}/.cargo/bin cargo build --release

strip: build
	strip --strip-all target/release/$(BINARY)

clean:
	env PATH=${PATH}:${HOME}/.cargo/bin cargo clean

install: strip
	test -d $(DESTDIR)/usr/bin || mkdir -m 0755 -p $(DESTDIR)/usr/bin
	install -m 0755 target/release/$(BINARY) $(DESTDIR)/usr/bin

uninstall:
	/bin/rm -f $(DESTDIR)/usr/bin/$(BINARY)

