#include <bits/stdc++.h>
using namespace std;

void printVec(vector<int> &v, int len){
	for(int i=0; i<len; ++i){
		cerr << v[i] << ", ";
	}
	cerr << endl;
}

int main(){
	string S;
	cin >> S;
	int N = S.length();

	vector<int> A(N); //A[i] <=> i番目の右側に何個Lが連続しているか(i番目も含む)
	vector<int> B(N); //B[i] <=> i番目の左側に何個Rが連続しているか(i番目も含む)
	vector<int> ans(N, 0);
	
	int count = 0;
	for(int i=0; i<N; ++i){
		if(S[i] == 'R'){
			++count;
		}
		else{
			count = 0;
		}
		B[i] = count;
	}
	
	count = 0;
	for(int i=N-1; i>=0; --i){
		if(S[i] == 'L'){
			++count;
		}
		else{
			count = 0;
		}
		A[i] = count;
	}
	
	//cerr << S << endl;
	//printVec(A, N);
	//printVec(B, N);

	/*
	  RRLRL
A:  0, 0, 1, 0, 1,
B:  1, 2, 0, 1, 0,
	*/

	for(int i=0; i<N-1; ++i){
		if(B[i] != 0 && B[i+1] == 0){
			ans[i] += (B[i]+1)/2;
			ans[i+1] += B[i]/2;
		}
	}

	for(int i=N-1; i>=1; --i){
		if(A[i] != 0 && A[i-1] == 0){
			ans[i] += (A[i]+1)/2;
			ans[i-1] += A[i]/2;
		}
	}


	for(int i=0; i<N; ++i){
		if(i != N-1){
			cout << ans[i] << " ";
		}
		else{
			cout << ans[i] << endl;
		}
	}
}

