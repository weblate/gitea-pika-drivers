all:
	true

install:
	mkdir -p $(DESTDIR)/usr/bin/
	cargo fetch
	cargo build --release
	cp -vf target/release/pika-drivers $(DESTDIR)/usr/bin/
	chmod 755 $(DESTDIR)/usr/bin/pika-drivers
	mkdir -p $(DESTDIR)/usr/lib/pika/drivers/
	cp data/*.sh $(DESTDIR)/usr/lib/pika/drivers/
	chmod 755 $(DESTDIR)/usr/bin/pika-drivers/*.sh
	mkdir -p $(DESTDIR)/usr/share/applications
	mkdir -p $(DESTDIR)/usr/share/icons/hicolor/scalable/apps
	cp -vf data/pika-drivers.svg $(DESTDIR)/usr/share/icons/hicolor/scalable/apps/
	cp -vf data/pika-drivers.desktop $(DESTDIR)/usr/share/applications/
