#include <bits/stdc++.h>
using namespace std;

int N, D;

int dis2(const vector<int> & p1, const vector<int> & p2){

	cerr << "(";
	for(int d=0; d<D; ++d){
		cerr << p1[d] << " ";
	}
	cerr << ")";
	cerr << endl;
	cerr << "(";
	for(int d=0; d<D; ++d){
		cerr << p2[d] << " ";
	}
	cerr << ")";

	int sum = 0;
	for(int d=0; d<D; ++d){
		int tmp = p1[d] - p2[d];
		tmp *= tmp;
		sum += tmp;
	}
	return sum;
}

int main(){
	cin >> N >> D;
	vector<vector<int> > data(N, vector<int>(D));
	for(int i=0; i<N; ++i){
		for(int j=0; j<D; ++j){
			int in;
			cin >> in;
			data[i][j] = in;
		}
	}

	int count = 0;
	for(int i=0; i<N; ++i){
		for(int j=0; j<i; ++j){
			cerr << "comparing i=" << i << "and j=" << j << endl;
			int dis_pow2 = dis2(data[i], data[j]);
			int dis = round(sqrt(dis_pow2));
			cerr << "dis*dis=" << (dis*dis) << ", dis_pow2=" << dis_pow2 << endl;
			if(dis*dis == dis_pow2){
				cerr << "count up" << endl;
			  ++count;
			}
		}
	}

	cout << count << endl;
}

