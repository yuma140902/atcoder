#include <bits/stdc++.h>
using namespace std;

int main(){
  int N;
  string tar;
  cin >> N >> tar;
  int tarlen = tar.size();
  
  vector<string> dat(N);
  for(int i=0; i<N; ++i){
    string in;
    cin >> in;
    dat[i] = in;
  }
  
  int count = 0;
  
  for(int i=0; i<N; ++i){
    
    string src = dat[i];
    int srclen = src.size();
    
    vector<int> margins;
    for(int margin=0; margin < srclen; ++margin){
      if(src[margin] == tar[0]){
        margins.push_back(margin);
      }
    }
    
    for(int j=0; j<margins.size(); ++j){
      int margin = margins[j];
      for(int step=1; margin + step*(tarlen-1) < srclen; ++step){
        char *buf = new char[tarlen+1];
        for(int k=0; k < tarlen; ++k){
          buf[k] = src[margin + step*k];
        }
        buf[tarlen] = '\0';
        //cout << "buf=" << buf << endl;
        if(tar == buf){
          //cout << "i: " << i << ", margin: " << margin << ", step: " << step << 
          //", buf=" << buf << endl;
          ++count;
          delete buf;
          goto nextsrc;
        }
        delete buf;
      }
    }
    
    nextsrc:;
  }
  
  cout << count << endl;
  
}

