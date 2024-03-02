#include <bits/stdc++.h>
using namespace std;

int main(){
  int N;
  cin >> N;
  
  vector<vector<int>> dat(N, vector<int>(3));
  vector<vector<bool>> table(101, vector<bool>(3, false));
  
  for(int i=0; i<N; ++i){
    vector<int> in(3);
    int n0, n1, n2;
    cin >> n0 >> n1 >> n2;
    in[0] = n0; in[1] = n1; in[2] = n2;
    
    for(int k=0; k<3; ++k){
      if(table[in[k]][k]) {
        for(int j=0; j<i; ++j){
          if(dat[j][k] == in[k]){
            dat[j][k] = 0;
            break;
          }
        }
        in[k] = 0;
      }
      else{
        table[in[k]][k] = true;
      }
    }
    
    dat[i][0] = in[0];
    dat[i][1] = in[1];
    dat[i][2] = in[2];
  }
  
  for(int i=0; i<N; ++i){
    cout << (dat[i][0] + dat[i][1] + dat[i][2]) << endl;
  }
}

