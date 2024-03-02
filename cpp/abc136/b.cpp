#include <bits/stdc++.h>
using namespace std;

int N;

int main(){
	cin >> N;

	int log = floor(log10((float)N));
	log = (log%2==0) ? log : log-1;
	int cand = min((int)pow(10, log+1)-1, N);

	//cerr << "log: " << log << endl;

	int count = 0;
	while(true){
		//cerr << "cand: " << cand << endl;
		if(cand != 0 && cand != 100 && cand != 10000 && cand != 1000000){
			//cerr << "passA" << endl;
			cand--;
			count++;
		}
		else if(log-2 >= 0){
			//cerr << "passB" << endl;
			count++;
			log -= 2;
			cand = (int)pow(10, log+1)-1;
		}
		else{
			//cerr << "passC" << endl;
			break;
		}
	}

	cout << count << endl;
}

