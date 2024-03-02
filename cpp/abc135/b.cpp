#include <bits/stdc++.h>
#include <cstdio>
using namespace std;
typedef long long int ll; // Long long

#define vi vector<int>
#define vii vector<vector<int> >
#define vi_(n) vector<int>((n))
#define vii_(n,m) vector<vector<int> >((n), vector<int>((m)))
#define vi_def(n, def) vector<int>((n), (def))
#define vii_def(n, m, def) vector<vector<int> >((n), vector<int>((m), (def)))

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1 i >= 0; --i)

int main(){
	int N;
	cin >> N;
	vi p(N);
	rep(i, N){
		int P;
		cin >> P;
		p[i] = P;
	}

	int cnt = 0;
	rep(i, N){
		if(p[i] != i+1){
			++cnt;
		}
	}

	cout << (cnt >= 3 ? "NO" : "YES") << endl;

}

