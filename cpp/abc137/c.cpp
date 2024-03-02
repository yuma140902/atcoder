#include <bits/stdc++.h>
using namespace std;

typedef long long int ll;

int N;
vector<string> s;
vector<vector<char> > chvv;

int main(){
	cin >> N;

	s = vector<string>(N);
	for(int i=0; i<N; ++i){
		string str;
		cin >> str;
		s[i] = str;
	}

	chvv = vector<vector<char> >();
	for(int i=0; i<N; ++i){
		string str = s[i];
		vector<char> chv;
		copy(str.begin(), str.end(), back_inserter(chv));
		sort(chv.begin(), chv.end());
		chvv.push_back(chv);
	}

	sort(chvv.begin(), chvv.end());
	chvv.push_back(vector<char>());

	ll lcount = 0;
	ll gcount = 0;
	vector<char> prev = chvv[0];
	for(int i=1; i<N+1; ++i){
		//cerr << "i: " << i << endl;
		if(prev == chvv[i]){
			//cerr << "pathA" << endl;
			//cerr << "lcount: " << lcount << endl;
			++lcount;
		}
		else{
			//cerr << "pathB" << endl;
			//cerr << "lcount: " << lcount << endl;
			if(lcount != 0){
				//cerr << "pathB2" << endl;
				//cerr << "diff: " << (lcount * (lcount+1) / 2) << endl;
				gcount += (lcount * (lcount+1) / 2); //nCr(n: lcount, r: 2);
				//cerr << "gcount: " << gcount << endl;
				lcount = 0;
			}
			prev = chvv[i];
		}
	}
	
	for(int i=0; i<N; ++i){
		for(int j=0; j<chvv[i].size(); ++j){
			//cerr << chvv[i][j];
		}
		//cerr << endl;
	}

	cout << gcount << endl;
}

