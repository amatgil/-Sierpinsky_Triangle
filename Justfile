buildcc:
	g++ -o libSierp.a -shared SierpinskyTriangle.cc

authenticate:
	eval `ssh-agent -s`
	ssh-add

build:
	just buildcc
	cargo rustc -- -L"." -l"Sierp"
run:
	just build
	LD_LIBRARY_PATH="." ./target/debug/imaging_fractals

