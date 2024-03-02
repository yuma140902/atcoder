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
#define per(i, n) for(int i=(n)-1; i >= 0; --i)

int main(){
	vi p(5);
	rep(i, 5){
		int in;
		cin >> in;
		p[i] = max(in, 40);
	}

	int ans = accumulate(p.begin(), p.end(), 0) / 5;
	cout << ans << endl;
}

