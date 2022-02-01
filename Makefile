all: out/linux.tar.lzma out/windows.zip

clean:
	rm -rf out build
	cargo clean

target/release/et-hs: src/*
	cargo build --release

target/x86_64-pc-windows-gnu/release/et-hs.exe: src/*
	cargo build --release --target x86_64-pc-windows-gnu

out:
	mkdir -p out

build/assets:
	mkdir -p build/assets

build/assets/et-hs.ldtk: assets/et-hs.ldtk build/assets
	cp -r assets/et-hs.ldtk build/assets/
	mkdir -p build/assets/et-hs
	cp -r assets/et-hs/*.ldtkl build/assets/et-hs/

build/assets/*.png: assets/*.png build/assets
	cp -r assets/*.png build/assets/

build/assets/atlas:
	mkdir -p build/assets/atlas
	cp assets/atlas/room_base.png build/assets/atlas/

build/et-hs.x86_64: build/assets target/release/et-hs
	cp target/release/et-hs build/et-hs.x86_64

build/et-hs.exe: build/assets target/x86_64-pc-windows-gnu/release/et-hs.exe
	cp target/x86_64-pc-windows-gnu/release/et-hs.exe build/et-hs.exe

out/linux.tar.lzma: build/assets/et-hs.ldtk build/assets/atlas build/et-hs.x86_64 out build/assets/*.png
	cd build && XZ_OPT=-9 tar --lzma -cf ../out/linux.tar.lzma assets et-hs.x86_64

out/windows.zip: build/assets/et-hs.ldtk build/assets/atlas build/et-hs.exe out build/assets/*.png
	cd build && zip -9 -r ../out/windows.zip assets et-hs.exe

