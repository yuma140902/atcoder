#include <bits/stdc++.h>
using namespace std;

int N;
vector<int> H;
vector<int> D; //delta

int main(){
	cin >> N;
	H = vector<int>(N);
	D = vector<int>(N-1);
	for(int i=0; i<N; ++i){
		int in;
		cin >> in;
		H[i] = in;
	}

	bool ok = true;

	int max = H[0];
	int min = H[0];
	for(int i=0; i<N; ++i){
		if(H[i] > max){
			max = H[i];
		}
		else /*if(H[i] < min)*/{
			min = std::min(min, H[i]);
			//cerr << "i:" << i << ", ";
			//cerr << "max-min=" << (max - min) << endl;
			if(max - min >= 2){
				ok = false;
				break;
			}
		}
	}

	
	for(int i=0; i<N-1; ++i){
		D[i] = H[i+1] - H[i];
	}

	for(int i=0; i<N-1; ++i){
		if(H[i] > H[i+1]){
			H[i] -= 1;
		}
	}

	for(int i=0; i<N-1; ++i){
		if(H[i] > H[i+1]){
			ok = false;
			break;
		}
	}

	int sum = accumulate(D.begin(), D.end(), 0);
	ok = ok && (sum >= 0);
	
	cout << (ok ? "Yes" : "No") << endl;
}

