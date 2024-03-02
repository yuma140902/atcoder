#include <bits/stdc++.h>
#include <cstdio>
using namespace std;
typedef long long int ll; // Long long

#define vi vector<int>
#define vii vector<vector<int> >
#define vi_(n) vector<int>((n))
#define vii_(n,m) vector<vector<int> >((n), vector<int>((m)))
#define vi__(n, def) vector<int>((n), (def))
#define vii__(n, m, def) vector<vector<int> >((n), vector<int>((m), (def)))

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1 i >= 0; --i)

int main(){
	int A, B;
	cin >> A >> B;
	if((A+B)&1 != 0){
		cout << "IMPOSSIBLE" << endl;
	}
	else{
		cout << (A+B)/2 << endl;
	}

}

