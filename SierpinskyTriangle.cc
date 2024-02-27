#include <iostream>
#include <vector>
#include <stdint.h>
using namespace std;

typedef vector<vector<unsigned char>> Matriu;

uint32_t exp(uint32_t n){
    uint32_t r = 1;
    for(uint32_t i = 0; i < n; ++i) {
        r *= 2;
    }
    return r;
}

Matriu sierpinsky1(uint32_t n) {
    Matriu matriu(exp(n-1), vector<unsigned char>(exp(n-1)));

    if(n == 1) {
        matriu[0][0] = '#';

    } else {
        Matriu matRecurs = sierpinsky1(n-1); 

        for(uint32_t i = 0; i < exp(n-2); ++i) {
            for(uint32_t j = 0; j < exp(n-2); ++j) {
                matriu[i][j] = matRecurs[i][j];
                matriu[i+exp(n-2)][j] = matRecurs[i][j];
                matriu[i+exp(n-2)][j+exp(n-2)] = matRecurs[i][j];
            }
        }
    }
    return matriu;
}

Matriu sierpinsky2(uint32_t n) {
    Matriu matriu(exp(n-1), vector<unsigned char>(exp(n), 1));

    if(n == 1) {
        matriu[0][0] = 2;
        matriu[0][1] = 3;
    } else {
        Matriu matRecurs = sierpinsky2(n-1); 

        for(uint32_t i = 0; i < exp(n-2); ++i) {
            for(uint32_t j = 0; j < exp(n-1); ++j) { 
                matriu[i][j+exp(n-2)] = matRecurs[i][j];
                matriu[i+exp(n-2)][j] = matRecurs[i][j];
                matriu[i+exp(n-2)][j+exp(n-1)] = matRecurs[i][j];
            }
        }
    }
    return matriu;
}

struct OneDArray {
  char* ptr;
  uint32_t sz;
};

OneDArray vecToOneDArray(std::vector<vector<char>> vec) {
	char* ptr = &vec[0][0];
	uint32_t sz = vec.size()*vec[0].size();
	return {ptr, sz}; 
}
OneDArray vecToOneDArray_flat(std::vector<char> vec) {
	OneDArray result;
	result.ptr = &vec[0];
	result.sz = vec.size();

	return result;
}

vector<char> flatten(vector<vector<unsigned char>> m ) {
	vector<char> outp;
	for (int y = 0; y < m.size(); ++y) {
		for (int x = 0; x < m[0].size(); ++x) {
			outp.push_back(m[y][x]);
		}
	}

	return outp;
}

extern "C" {
	OneDArray sierpinsky_extern(uint32_t n) {
		printf("Mathing...\n");
		vector<vector<unsigned char>> m = sierpinsky2(n);
		vector<char> j = flatten(m);

		OneDArray result;
		result.ptr = &j[0];
		result.sz = j.size();

		//for (int i = 0; i < result.sz; ++i) {
		//	printf("i = %d: %d\n",i, *(result.ptr + i));
		//}
		printf("@: %p\n", result.ptr);
		printf("sz: %d\n", result.sz);

		return result;
	}
}


