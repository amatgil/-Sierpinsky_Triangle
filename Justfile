buildcc:
	g++ -o libSierp.a -shared SierpinskyTriangle.cc

build:
	just buildcc
	cargo rustc -- -L"." -l"Sierp"
run n:
	just build
	LD_LIBRARY_PATH="." ./target/debug/imaging_fractals {{n}}

