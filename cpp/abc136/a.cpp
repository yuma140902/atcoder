#include <bits/stdc++.h>
using namespace std;

int A, B, C;

int main(){
	cin >> A >> B >> C;
	int ans = C-(A-B);
	ans = (ans < 0) ? 0 : ans;
	cout << ans << endl;
}

