#include <bits/stdc++.h>
using namespace std;

int main(){
	int A, B;
	cin >> A >> B;
	int m = max(A+B, A-B);
	m = max(m, A*B);

	cout << m << endl;
}

