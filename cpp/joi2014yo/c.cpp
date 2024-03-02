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

int min(int x1, int y1, int x2, int y2){
	int xd = abs(x1-x2);
	int yd = abs(y1-y2);

	if(x1 >= x2 && y1 >= y2){
		return std::max(xd, yd); // == xd + yd - std::min(xd, yd);
	}
	else if(x1 <= x2 && y1 <= y2){
		return std::max(xd, yd);
	}
	else if(x1 >= x2 && y1 <= y2){
		return xd + yd;
	}
	else{
		return xd + yd;
	}
}

int main(){
	int W, H, N;
	cin >> W >> H >> N;
	vi PX(N);
	vi PY(N);
	rep(i, N){
		int px, py;
		cin >> px >> py;
		PX[i] = px;
		PY[i] = py;
	}

	int ans = 0;
	rep(i, N-1){
		ans += min(PX[i], PY[i], PX[i+1], PY[i+1]);
	}

	cout << ans << endl;
}

