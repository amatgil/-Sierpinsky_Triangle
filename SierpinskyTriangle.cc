#include <iostream>
#include <vector>
using namespace std;

typedef vector<vector<char>> Matriu;

int exp(int n){
    int r = 1;
    for(int i = 0; i < n; ++i) {
        r *= 2;
    }
    return r;
}

Matriu sierpinsky1(int n) {
    Matriu matriu(exp(n-1), vector<char>(exp(n-1)));

    if(n == 1) {
        matriu[0][0] = '@';

    } else {
        Matriu matRecurs = sierpinsky1(n-1); 

        for(int i = 0; i < exp(n-2); ++i) {
            for(int j = 0; j < exp(n-2); ++j) {
                matriu[i][j] = matRecurs[i][j];
                matriu[i+exp(n-2)][j] = matRecurs[i][j];
                matriu[i+exp(n-2)][j+exp(n-2)] = matRecurs[i][j];
            }
        }
    }
    return matriu;
}

Matriu sierpinsky2(int n) {
    Matriu matriu(exp(n-1), vector<char>(exp(n)));

    if(n == 1) {
        matriu[0][0] = '/';
        matriu[0][1] = '\\';
    } else {
        Matriu matRecurs = sierpinsky2(n-1); 

        for(int i = 0; i < exp(n-2); ++i) {
            for(int j = 0; j < exp(n-1); ++j) { 
                matriu[i][j+exp(n-2)] = matRecurs[i][j];
                matriu[i+exp(n-2)][j] = matRecurs[i][j];
                matriu[i+exp(n-2)][j+exp(n-1)] = matRecurs[i][j];
            }
        }
    }
    return matriu;
}

int main() {
    int n;
    cin >> n;
    Matriu mat = sierpinsky1(n);

    for(int i = 0; i < exp(n-1); ++i) {
        for(int j = 0; j < exp(n-1); ++j) {
            if(int(mat[i][j]) == 0) cout << "  ";
            else cout << "|\\";
        }
        cout << endl;
    }

    cout << "\n\n\n\n\n\n";
    mat = sierpinsky2(n);

    for(int i = 0; i < exp(n-1); ++i) {
        for(int j = 0; j < exp(n); ++j) {
            if(int(mat[i][j]) == 0) cout << " ";
            else cout << mat[i][j];
        }
        cout << endl;
    }
}

/*

       /\
      /\/\
     /\  /\
    /\/\/\/\
   /\
  /\/\
 /\  /\
/\/\/\/\/\/\/\/\

2*1
4*2
8*4




*/