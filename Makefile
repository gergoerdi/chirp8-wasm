OUTDIR	 = _deploy

all:
	wasm-pack build --release --target  web
	mkdir -p $(OUTDIR)
	cp -fax -t $(OUTDIR) pkg www index.html
