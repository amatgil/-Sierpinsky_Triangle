buildr:
	g++ -o libSierp.a -shared SierpinskyTriangle.cc
	rustc -L"." -l"Sierp" src/main.rs
