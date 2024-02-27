buildcc:
	g++ -o libSierp.a -shared SierpinskyTriangle.cc

buildr:
	just buildcc
	rustc -L"." -l"Sierp" src/main.rs

build:
	just buildcc
	cargo rustc -- -L"." -l"Sierp"
run:
	just build
	LD_LIBRARY_PATH="." ./target/debug/imaging_fractals

